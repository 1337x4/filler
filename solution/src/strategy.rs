use crate::board::{Board, Cell};

/// Find the best (x, y) placement for the piece on the board.
/// Strategy: greedy — prefer placements that expand territory towards
/// the center (minimise distance from new cells to board center),
/// maximising area gain and pushing toward opponent.
pub fn find_best_placement(board: &Board, piece: &[Vec<bool>], cols: usize, rows: usize) -> (usize, usize) {
    let p_rows = piece.len();
    let p_cols = if p_rows > 0 { piece[0].len() } else { return (0, 0); };

    let center_r = rows as f64 / 2.0;
    let center_c = cols as f64 / 2.0;

    let mut best_score: f64 = f64::NEG_INFINITY;
    let mut best_pos: Option<(usize, usize)> = None;

    for py in 0..rows {
        for px in 0..cols {
            if !board.is_valid_placement(piece, px, py) {
                continue;
            }

            let score = score_placement(board, piece, px, py, p_rows, p_cols, center_r, center_c);

            if score > best_score {
                best_score = score;
                best_pos = Some((px, py));
            }
        }
    }

    best_pos.unwrap_or((0, 0))
}

/// Score a placement: reward cells closer to opponent territory,
/// penalise placements near edges.
fn score_placement(
    board: &Board,
    piece: &[Vec<bool>],
    px: usize,
    py: usize,
    p_rows: usize,
    p_cols: usize,
    center_r: f64,
    center_c: f64,
) -> f64 {
    let mut score = 0.0f64;

    for r in 0..p_rows {
        for c in 0..p_cols {
            if !piece[r][c] { continue; }
            let br = (py + r) as f64;
            let bc = (px + c) as f64;

            // Distance from center — lower is better (closer to middle = more expansion room)
            let dist_center = ((br - center_r).powi(2) + (bc - center_c).powi(2)).sqrt();
            score -= dist_center * 0.5;

            // Bonus for adjacency to opponent cells (aggressive expansion)
            let br_i = (py + r) as isize;
            let bc_i = (px + c) as isize;
            for (dr, dc) in [(-1,0),(1,0),(0,-1),(0,1)] {
                let nr = br_i + dr;
                let nc = bc_i + dc;
                if nr >= 0 && nc >= 0 && (nr as usize) < board.rows && (nc as usize) < board.cols {
                    if board.grid[nr as usize][nc as usize] == Cell::Opponent {
                        score += 3.0;
                    }
                }
            }
        }
    }

    score
}
