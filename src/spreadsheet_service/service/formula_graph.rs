use super::super::models;
use rstar::{Envelope, Point, PointDistance, RTree, RTreeObject, AABB};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub struct FormulaGraph {
    rt: RTree<RTreeNode>,

    dependents_map: HashMap<models::CellRange, HashSet<models::CellLocation>>,
    dependencies_map: HashMap<models::CellLocation, HashSet<models::CellRange>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct RTreeNode {
    cell: models::CellLocation,
    points_to: models::CellRange,
}

pub struct InsertResult {
    pub inserted_cells: Vec<models::CellLocation>,
    pub circular_cells: Vec<models::CellLocation>,
}

impl RTreeObject for RTreeNode {
    type Envelope = rstar::AABB<models::CellLocation>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_corners(
            models::CellLocation {
                row: self.points_to.start_row,
                col: self.points_to.start_col,
            },
            models::CellLocation {
                row: self.points_to.stop_row,
                col: self.points_to.stop_col,
            },
        )
    }
}

impl Point for models::CellLocation {
    type Scalar = i32;
    const DIMENSIONS: usize = 2;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        models::CellLocation {
            row: generator(0),
            col: generator(1),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        match index {
            0 => self.row,
            1 => self.col,
            _ => unreachable!(),
        }
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        match index {
            0 => &mut self.row,
            1 => &mut self.col,
            _ => unreachable!(),
        }
    }
}

impl RTreeNode {
    /// Returns the nearest point within this rectangle to a given point.
    ///
    /// If `query_point` is contained within this rectangle, `query_point` is returned.
    pub fn nearest_point(&self, query_point: models::CellLocation) -> models::CellLocation {
        let aabb = AABB::from_corners(
            [self.points_to.start_row, self.points_to.start_col],
            [self.points_to.stop_row, self.points_to.stop_col],
        );

        let p = aabb.min_point(&[query_point.row, query_point.col]);
        models::CellLocation {
            row: p[0],
            col: p[1],
        }
    }
}

impl PointDistance for RTreeNode {
    fn distance_2(
        &self,
        point: &<Self::Envelope as rstar::Envelope>::Point,
    ) -> <<Self::Envelope as rstar::Envelope>::Point as Point>::Scalar {
        let nearest = self.nearest_point(point.clone());
        models::CellLocation {
            row: nearest.row - point.row,
            col: nearest.col - point.col,
        }
        .magnitude()
    }

    fn contains_point(&self, point: &<Self::Envelope as rstar::Envelope>::Point) -> bool {
        let aabb = AABB::from_corners(
            models::CellLocation {
                row: self.points_to.start_row,
                col: self.points_to.start_col,
            },
            models::CellLocation {
                row: self.points_to.stop_row,
                col: self.points_to.stop_col,
            },
        );
        aabb.contains_point(point)
    }

    fn distance_2_if_less_or_equal(
        &self,
        point: &<Self::Envelope as rstar::Envelope>::Point,
        max_distance_2: <<Self::Envelope as rstar::Envelope>::Point as Point>::Scalar,
    ) -> Option<<<Self::Envelope as rstar::Envelope>::Point as Point>::Scalar> {
        let distance_2 = self.distance_2(point);
        if distance_2 <= max_distance_2 {
            Some(distance_2)
        } else {
            None
        }
    }
}

impl FormulaGraph {
    pub fn new() -> FormulaGraph {
        FormulaGraph {
            rt: RTree::new_with_params(),
            dependents_map: HashMap::new(),
            dependencies_map: HashMap::new(),
        }
    }

    // insert_cell inserts the provided cell into the formula graph
    // and returns a stack of the affected cell ranges.
    // These cell ranges should be recomputed by popping from this stack.
    // Cells lower in the stack may depend on cells evaluated higher in it.
    pub fn insert_cell(
        &mut self,
        cell: models::Cell,
        dependencies: Vec<models::CellRange>,
    ) -> InsertResult {
        println!("insert cell {:?}", cell);

        // Delete any existing dependencies this cell has marked
        for (_, deps) in &self.dependencies_map {
            for dep in deps {
                let to_delete = RTreeNode {
                    cell: cell.loc(),
                    points_to: dep.clone(),
                };
                if self.rt.contains(&to_delete) {
                    println!("deleting RTreeNode: {:?}", to_delete);
                    self.rt.remove(&to_delete);
                }
                (*self.dependents_map.entry(*dep).or_insert(HashSet::new())).remove(&cell.loc());
            }
        }

        // For each dependency:
        // 1) Mark the dependency and insert an RTree node containing the dependency's boundaries.
        // 2) Mark the inserted cell as a dependent of the dependency
        for d in dependencies {
            let to_insert = RTreeNode {
                cell: cell.loc(),
                points_to: d.clone(),
            };
            println!("inserting rtree node: {:?}", to_insert);
            self.rt.insert(to_insert);
            (*self
                .dependencies_map
                .entry(cell.loc())
                .or_insert(HashSet::new()))
            .insert(d);

            (*self.dependents_map.entry(d).or_insert(HashSet::new())).insert(cell.loc());
        }

        // Find all the dependents and update dependents map for the inserted cell
        let mut existing: Vec<models::CellLocation> = self
            .rt
            .locate_all_at_point(&cell.loc())
            .map(|e| e.cell)
            .collect();
        println!("initial dfs for {:?}", cell.loc());
        println!("initial dependents are {:?}", existing);

        let res = self.cells_to_eval(cell.loc(), &existing);

        for e in &existing {
            println!("found existing: {:?}", e);
            if *e == cell.loc() {
                continue;
            }
            (*self
                .dependents_map
                .entry(cell.to_range().clone())
                .or_insert(HashSet::new()))
            .insert(e.clone());
        }

        res
    }

    // cells_to_eval returns the list of cells which must be evaluated in order to evaluate
    // the provided cell's formula.
    fn cells_to_eval(
        &self,
        cell_loc: models::CellLocation,
        dependents: &Vec<models::CellLocation>,
    ) -> InsertResult {
        let mut visited_set = HashSet::new();
        let mut stack = vec![];
        let mut circular = vec![];

        println!("initial stack is {:?}", stack);
        self.dfs(
            &mut dependents.clone(),
            &mut stack,
            &mut visited_set,
            &mut vec![cell_loc],
            &mut circular,
        );
        let mut ret = vec![];

        println!("cell_to_eval stack is {:?}", stack);
        for s in stack.iter() {
            ret.push(models::CellLocation {
                row: s.row,
                col: s.col,
            });
        }

        println!(
            "insert complete. Circular cells {:?} non-circular {:?}",
            circular, stack
        );
        if !circular.contains(&cell_loc) {
            ret.push(cell_loc);
        }
        InsertResult {
            inserted_cells: ret.clone(),
            circular_cells: circular.clone(),
        }
    }

    fn dfs(
        &self,
        deps: &mut Vec<models::CellLocation>,
        stack: &mut Vec<models::CellLocation>,
        visited_set: &mut HashSet<models::CellLocation>,
        path: &mut Vec<models::CellLocation>,
        circular: &mut Vec<models::CellLocation>,
    ) {
        println!("dfs dependents are {:?}", deps);
        if deps.len() == 0 {
            return;
        }
        for d in deps {
            if circular.contains(d) {
                continue;
            }
            if (&path).contains(d) {
                println!("path: {:?}", path);
                for c in path.iter() {
                    println!("adding {:?} to circulars", c);
                    circular.push(c.clone());
                    visited_set.insert(c.clone());
                }
            } else {
                path.push(d.clone());
                println!("dfs for {:?}", d.to_range());
                self.dfs(
                    &mut match self.dependents_map.get(&d.to_range()) {
                        Some(hs) => hs.clone().into_iter().collect(),
                        None => vec![],
                    },
                    stack,
                    visited_set,
                    path,
                    circular,
                );
            }
            path.pop();
            visited_set.insert(d.clone());
            stack.push(d.clone());
        }
    }
}
