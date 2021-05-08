#[cfg(test)]
mod tests {
    pub use super::super::super::models::*;
    pub use super::super::formula_graph::FormulaGraph;

    #[test]
    fn test_insert_cell() {
        let mut fg = FormulaGraph::new();

        // Simple dependency
        // A1 --> B1
        let a1 = Cell {
            row: 0,
            col: 0,
            value: "=B1".to_string(),
            display_value: "0".to_string(),
        };
        let b1 = Cell {
            row: 0,
            col: 1,
            value: "10".to_string(),
            display_value: "10".to_string(),
        };
        let mut to_eval = fg.insert_cell(b1.clone(), vec![]);
        assert_eq!(Vec::<CellLocation>::new(), to_eval);

        to_eval = fg.insert_cell(a1.clone(), vec![b1.to_range()]);
        assert_eq!(Vec::<CellLocation>::new(), to_eval);

        to_eval = fg.insert_cell(b1.clone(), vec![]);
        assert_eq!(vec![a1.loc()], to_eval);

        // Add a third dependency, A1 --> B1 --> C1
        let c1 = Cell {
            row: 0,
            col: 2,
            value: "20".to_string(),
            display_value: "20".to_string(),
        };

        to_eval = fg.insert_cell(c1.clone(), vec![]);
        assert_eq!(Vec::<CellLocation>::new(), to_eval);

        // Add the dependency on cell C1. A1 should be recomputed
        to_eval = fg.insert_cell(b1.clone(), vec![c1.to_range()]);
        assert_eq!(vec![a1.loc()], to_eval);

        // Modify C1. B1 and A1 should be recomputed in that order
        to_eval = fg.insert_cell(c1.clone(), vec![]);
        assert_eq!(2, to_eval.len());
        assert_eq!(b1.loc(), to_eval.pop().unwrap());
        assert_eq!(a1.loc(), to_eval.pop().unwrap());
    }
}
