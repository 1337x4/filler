use crate::board::{Board, Cell};

/// Parse the initial header line, returning player number (1 or 2).
/// Format: `$$$ exec p<n> : [<path>]`
pub fn parse_header(line: &str) -> u8 {
    // Find "p1" or "p2"
    if let Some(pos) = line.find(" p") {
        let after = &line[pos + 2..];
        if after.starts_with('1') {
            return 1;
        } else if after.starts_with('2') {
            return 2;
        }
    }
    1
}

/// Parse "Anfield <cols> <rows>:" returning (cols, rows).
pub fn parse_anfield_dimensions(line: &str) -> (usize, usize) {
    // Format: "Anfield <cols> <rows>:"
    let parts: Vec<&str> = line.split_whitespace().collect();
    let cols = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let rows = parts.get(2).and_then(|s| s.trim_end_matches(':').parse().ok()).unwrap_or(0);
    (cols, rows)
}

/// Parse "Piece <rows> <cols>:" returning (rows, cols).
pub fn parse_piece_dimensions(line: &str) -> (usize, usize) {
    // Format: "Piece <rows> <cols>:"
    let parts: Vec<&str> = line.split_whitespace().collect();
    let rows = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let cols = parts.get(2).and_then(|s| s.trim_end_matches(':').parse().ok()).unwrap_or(0);
    (rows, cols)
}

/// Parse the Anfield grid rows into a Board.
/// Each row looks like: `000 ..@..$....`
/// player: 1 => own chars are 'a'/'@', opponent 's'/'$'
/// player: 2 => own chars are 's'/'$', opponent 'a'/'@'
pub fn parse_anfield(raw_rows: &[String], cols: usize, rows: usize, player: u8) -> Board {
    let own_chars: &[char] = if player == 1 { &['a', '@'] } else { &['s', '$'] };
    let opp_chars: &[char] = if player == 1 { &['s', '$'] } else { &['a', '@'] };

    let mut grid = vec![vec![Cell::Empty; cols]; rows];

    for (r, line) in raw_rows.iter().enumerate() {
        if r >= rows { break; }
        // Strip the row-number prefix (e.g. "000 ") — find the first space and take rest
        let content = if let Some(idx) = line.find(' ') {
            &line[idx + 1..]
        } else {
            line.as_str()
        };
        for (c, ch) in content.chars().enumerate() {
            if c >= cols { break; }
            grid[r][c] = if own_chars.contains(&ch) {
                Cell::Own
            } else if opp_chars.contains(&ch) {
                Cell::Opponent
            } else {
                Cell::Empty
            };
        }
    }

    Board { grid, rows, cols }
}

/// Parse piece rows into a 2D bool grid (true = filled cell).
pub fn parse_piece(raw_rows: &[String], rows: usize, cols: usize) -> Vec<Vec<bool>> {
    let mut piece = vec![vec![false; cols]; rows];
    for (r, line) in raw_rows.iter().enumerate() {
        if r >= rows { break; }
        for (c, ch) in line.chars().enumerate() {
            if c >= cols { break; }
            if ch == '#' || ch == 'O' {
                piece[r][c] = true;
            }
        }
    }
    piece
}
