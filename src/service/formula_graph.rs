use super::super::models;
use super::super::models::context::EvalContext;
use super::super::parser;
use rstar::RTree;
use rstar::RTreeObject;

pub struct FormulaGraph {
    rt: RTree<models::CellRange>,

    dependents_map: std::collections::HashMap<models::CellRange, models::CellLocation>,
    dependencies_map: std::collections::HashMap<models::CellLocation, models::CellRange>,
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
    // insert_cell inserts the provided cell into the formula graph and returns the affected
    // cell ranges. These cell ranges should be recomputed
    fn insert_cell(
        &mut self,
        cell: models::Cell,
        dependencies: Vec<models::CellRange>,
    ) -> Vec<models::CellRange> {
        let range = cell.to_range();
        if !self.rt.contains(&range) {
            // cell is not yet in the R-Tree
            self.rt.insert(range.clone());
        }
        for d in dependencies {
            self.dependencies_map.insert(cell.loc(), d);
        }

        // Find all the dependents and update dependencies map for the inserted cell
        let existing = self.rt.locate_in_envelope(&range.envelope());
        let mut ret = vec![];

        for e in existing {
            self.dependents_map.insert(e.clone(), cell.loc());
            ret.push(e.clone());
        }
        ret
    }
}
