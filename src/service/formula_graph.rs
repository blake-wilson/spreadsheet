use super::super::models;
use super::super::models::context::EvalContext;
use super::super::parser;
use rstar::RTree;
use rstar::RTreeObject;
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

impl rstar::RTreeObject for RTreeNode {
    type Envelope = rstar::AABB<[i32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_corners(
            [self.points_to.start_row, self.points_to.start_col],
            [self.points_to.stop_row, self.points_to.stop_col],
        )
    }
}

impl rstar::PointDistance for RTreeNode {
    fn distance_2(&self, point: &[i32; 2]) -> i32 {
        let dist = ((self.points_to.start_row + self.points_to.stop_row) / 2 - point[0])
            + ((self.points_to.start_col + self.points_to.stop_col) / 2 - point[1]);
        dist * dist
    }
}

impl rstar::RTreeObject for models::CellRange {
    type Envelope = rstar::AABB<[i32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_corners(
            [self.start_row, self.start_col],
            [self.stop_row, self.stop_col],
        )
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
            self.rt.insert(to_insert);
            (*self
                .dependencies_map
                .entry(cell.loc())
                .or_insert(HashSet::new()))
            .insert(d);

            (*self.dependents_map.entry(d).or_insert(HashSet::new())).insert(cell.loc());
        }
        let self_ref = RTreeNode {
            cell: cell.loc(),
            points_to: cell.to_range(),
        };
        self.rt.insert(self_ref);

        // Find all the dependents and update dependents map for the inserted cell
        let cr = cell.to_range();
        let existing = self.rt.locate_all_at_point(&[cr.start_row, cr.start_col]);

        for e in existing {
            println!("found {:?} as a dependency", e);
            if e.cell == cell.loc() {
                continue;
            }
            println!("add to dependents map: {:?}", e.cell);
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
