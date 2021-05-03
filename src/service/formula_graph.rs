use super::super::models;
use super::super::models::context::EvalContext;
use super::super::parser;
use rstar::RTree;
use rstar::RTreeObject;
use std::collections::HashMap;

pub struct FormulaGraph {
    rt: RTree<models::CellRange>,

    dependents_map: HashMap<models::CellRange, Vec<models::CellLocation>>,
    dependencies_map: HashMap<models::CellLocation, Vec<models::CellRange>>,
}

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
struct SearchNode {
    loc: models::CellLocation,
    visited: bool,
}

impl SearchNode {
    fn new_list(locs: Vec<models::CellLocation>) -> Vec<SearchNode> {
        let mut ret = vec![];
        for l in locs {
            ret.push(SearchNode {
                loc: l,
                visited: false,
            })
        }
        ret
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
    fn insert_cell(
        &mut self,
        cell: models::Cell,
        dependencies: Vec<models::CellRange>,
    ) -> Vec<models::CellLocation> {
        let range = cell.to_range();
        if !self.rt.contains(&range) {
            // cell is not yet in the R-Tree
            self.rt.insert(range.clone());
        }
        for d in dependencies {
            self.rt.insert(d.clone());
            (*self.dependencies_map.entry(cell.loc()).or_insert(vec![])).push(d);
        }

        // Find all the dependents and update dependencies map for the inserted cell
        let existing = self.rt.locate_in_envelope(&range.envelope());

        for e in existing {
            (*self.dependents_map.entry(e.clone()).or_insert(vec![])).push(cell.loc());
        }

        self.cells_to_eval(&cell)
    }

    // cells_to_eval returns the list of cells which must be evaluated in order to evaluate
    // the provided cell's formula.
    fn cells_to_eval(&self, cell: &models::Cell) -> Vec<models::CellLocation> {
        let search_nodes = self.dfs(&cell.loc(), &vec![]);
        let mut ret = vec![];
        for s in search_nodes {
            ret.push(models::CellLocation {
                row: s.loc.row,
                col: s.loc.col,
            });
        }
        ret
    }

    fn dfs(&self, cell: &models::CellLocation, formula_stack: &Vec<SearchNode>) -> Vec<SearchNode> {
        let formula_stack = self.dependents_map[&cell.to_range().clone()].clone();
        let mut search_lst = SearchNode::new_list(formula_stack);
        for i in 0..search_lst.len() {
            let s = search_lst[i].clone();
            if !s.visited {
                self.dfs(&s.loc, &search_lst);
            }
            search_lst.push(s.clone());
        }
        search_lst
    }
}
