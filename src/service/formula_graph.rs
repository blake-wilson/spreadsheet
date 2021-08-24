use super::super::models;
use rstar::{Envelope, Point, PointDistance, RTree, RTreeObject, AABB};
use std::collections::{HashMap, HashSet};

pub struct FormulaGraph {
    rt: RTree<RTreeNode>,

    dependents_map: HashMap<models::CellLocation, HashSet<models::CellLocation>>,
    dependencies_map: HashMap<models::CellLocation, HashSet<models::CellRange>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct RTreeNode {
    cell: models::CellLocation,
    points_to: models::CellRange,
}

#[derive(Debug, PartialEq)]
pub struct InsertResult {
    pub inserted_cells: Vec<models::CellLocation>,
    pub circular: bool,
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
                row: self.points_to.stop_row - 1,
                col: self.points_to.stop_col - 1,
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
        println!(
            "insert cell {:?} with dependencies {:?}\n",
            cell, dependencies
        );

        // Delete any existing dependencies this cell has marked
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
        }

        // Find all the dependents and update dependents map for the inserted cell
        let mut existing: Vec<RTreeNode> = Vec::new();
        for e in self.rt.locate_all_at_point(&cell.loc()) {
            existing.push(e.clone())
        }

        for e in existing {
            println!("found existing: {:?}", e);
            if e.cell == cell.loc() {
                // Clean up existing dependencies
                let cpy = e.clone();
                self.rt.remove(&cpy);
                continue;
            }
            (*self
                .dependents_map
                .entry(cell.loc().clone())
                .or_insert(HashSet::new()))
            .insert(e.cell);
            println!(
                "dependents map is now {:?}",
                self.dependents_map.get(&cell.loc())
            );
        }

        println!("cells to eval for cell: {:?}\n", cell);
        self.cells_to_eval(&cell)
    }

    // cells_to_eval returns the list of cells which must be evaluated in order to evaluate
    // the provided cell's formula.
    fn cells_to_eval(&self, cell: &models::Cell) -> InsertResult {
        let mut stack = vec![];
        let mut visited_set = HashSet::new();
        let mut circular = false;
        self.dfs(
            &cell.loc(),
            &mut stack,
            &mut visited_set,
            &mut vec![cell.loc()],
            &mut circular,
        );
        let mut ret = vec![];
        println!("stack {:?}", stack);
        for s in stack.iter() {
            ret.push(models::CellLocation {
                row: s.row,
                col: s.col,
            });
        }

        println!("insert complete. Circular? {:?}", circular);
        if circular {
            stack = visited_set.into_iter().collect();
        }
        InsertResult {
            inserted_cells: stack.clone(),
            circular,
        }
    }

    fn dfs(
        &self,
        cell: &models::CellLocation,
        stack: &mut Vec<models::CellLocation>,
        visited_set: &mut HashSet<models::CellLocation>,
        path: &mut Vec<models::CellLocation>,
        circular: &mut bool,
    ) {
        println!("get dependents for {:?}", cell);
        let dependents = self.dependents_map.get_key_value(&cell.clone());
        println!("dependents: {:?}", dependents);

        match dependents {
            Some((_, deps)) => {
                println!("formula stack {:?}", stack);
                for d in deps {
                    if (&path).contains(d) {
                        println!("path: {:?}", path);
                        for c in path.iter() {
                            println!("adding {:?} to circulars", c);
                            visited_set.insert(c.clone());
                        }
                        *circular = true
                    }
                    path.push(d.clone());
                    if !visited_set.contains(d) {
                        self.dfs(&d, stack, visited_set, path, circular);
                    }
                    path.pop();
                    visited_set.insert(d.clone());
                    stack.push(d.clone());
                }
            }
            None => {}
        }
        println!("returning\n\n");
    }
}
