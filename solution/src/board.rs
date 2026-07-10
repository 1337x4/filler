#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Own,
    Opponent,
}

#[derive(Clone, Debug)]
pub struct Board {
    pub grid: Vec<Vec<Cell>>,
    pub rows: usize,
    pub cols: usize,
}

impl Board {
    /// Returns true if placing `piece` at board position (px, py)
    /// results in exactly one overlap with own territory and zero with opponent.
    pub fn is_valid_placement(&self, piece: &[Vec<bool>], px: usize, py: usize) -> bool {
        let p_rows = piece.len();
        let p_cols = if p_rows > 0 { piece[0].len() } else { return false; };

        // Boundary check
        if py + p_rows > self.rows || px + p_cols > self.cols {
            return false;
        }

        let mut own_overlaps = 0usize;
        let mut opp_overlaps = 0usize;

        for (r, row) in piece.iter().enumerate() {
            for (c, &filled) in row.iter().enumerate() {
                if !filled { continue; }
                let br = py + r;
                let bc = px + c;
                match self.grid[br][bc] {
                    Cell::Own => own_overlaps += 1,
                    Cell::Opponent => opp_overlaps += 1,
                    Cell::Empty => {}
                }
            }
        }

        own_overlaps == 1 && opp_overlaps == 0
    }
}
