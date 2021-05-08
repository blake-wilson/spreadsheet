use super::super::models;
use rstar::{Envelope, Point, PointDistance, RTree, RTreeObject, AABB};
use std::collections::{HashMap, HashSet};

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

    // insert_cell inserts the provided cell into the formula graph and returns the affected
    // cell ranges. These cell ranges should be recomputed in the order they are returned.
    // Cells later in the returned list may depend on cells evaluated earlier in the list.
    pub fn insert_cell(
        &mut self,
        cell: models::Cell,
        dependencies: Vec<models::CellRange>,
    ) -> Vec<models::CellLocation> {
        println!("insert cell {:?}", cell);

        // For each dependency:
        // 1) Mark the dependency and insert an RTree node containing the dependencies boundaries.
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
        let existing = self.rt.locate_all_at_point(&cell.loc());

        for e in existing {
            println!("found existing: {:?}", e);
            if e.cell == cell.loc() {
                continue;
            }
            (*self
                .dependents_map
                .entry(cell.to_range().clone())
                .or_insert(HashSet::new()))
            .insert(e.cell);
        }

        self.cells_to_eval(&cell)
    }

    // cells_to_eval returns the list of cells which must be evaluated in order to evaluate
    // the provided cell's formula.
    fn cells_to_eval(&self, cell: &models::Cell) -> Vec<models::CellLocation> {
        let mut stack = vec![];
        self.dfs(&cell.loc(), &mut stack, &mut HashSet::new());
        println!("formula stack: {:?}", stack);
        let mut ret = vec![];
        for s in stack {
            ret.push(models::CellLocation {
                row: s.row,
                col: s.col,
            });
        }
        ret
    }

    fn dfs(
        &self,
        cell: &models::CellLocation,
        stack: &mut Vec<models::CellLocation>,
        visited_set: &mut HashSet<models::CellLocation>,
    ) {
        println!("get dependents for {:?}", cell);
        let dependents = self.dependents_map.get_key_value(&cell.to_range().clone());
        println!("dependents: {:?}", dependents);
        match dependents {
            Some((_, deps)) => {
                println!("formula stack {:?}", stack);
                for d in deps {
                    if !visited_set.contains(d) {
                        self.dfs(&d, stack, visited_set);
                    }
                    visited_set.insert(d.clone());
                    stack.push(d.clone());
                }
            }
            None => {}
        }
    }
}
