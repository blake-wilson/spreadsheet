use super::super::models;
use super::super::models::context::EvalContext;
use super::super::parser;
use rstar::RTree;

enum CellOrRange {
    Cell(models::Cell),
    Range(models::CellRange),
}

pub struct CellNode {
    // dependents are the cells or cell ranges which this cell depends on
    dependents: Vec<Box<CellNode>>,
    // dependencies are the cells or cell ranges which depend on this cell
    dependencies: Vec<Box<CellNode>>,

    range: CellOrRange,
}

pub struct FormulaGraph {
    rt: RTree<CellNode>,
}

impl rstar::RTreeObject for CellNode {
    type Envelope = rstar::AABB<[i32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        match &self.range {
            CellOrRange::Cell(c) => rstar::AABB::from_point([c.row, c.col]),
            CellOrRange::Range(r) => {
                rstar::AABB::from_corners([r.start_row, r.start_col], [r.stop_row, r.stop_col])
            }
        }
    }
}

impl FormulaGraph {
    fn insert_cell(&mut self, cell: models::Cell) {
        self.rt.insert()
    }
}
