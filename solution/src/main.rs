mod parser;
mod board;
mod strategy;

#[cfg(test)]
mod tests;

use std::io::{self, BufRead, Write};
use parser::{parse_header, parse_anfield, parse_piece};
use strategy::find_best_placement;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut lines_iter = stdin.lock().lines().map(|l| l.expect("Failed to read line"));

    // First line: $$$ exec p<n> : [<path>]
    let header_line = lines_iter.next().expect("Expected header line");
    let player = parse_header(&header_line);

    loop {
        // Read Anfield header: "Anfield <cols> <rows>:"
        let anfield_header = match lines_iter.next() {
            Some(l) => l,
            None => break,
        };
        if anfield_header.trim().is_empty() {
            // try next
            continue;
        }

        let (cols, rows) = parser::parse_anfield_dimensions(&anfield_header);

        // Read rows+1 lines (first is column index header, then rows of grid)
        let mut raw_rows: Vec<String> = Vec::with_capacity(rows);
        // skip the column-index line
        let _ = lines_iter.next();
        for _ in 0..rows {
            match lines_iter.next() {
                Some(l) => raw_rows.push(l),
                None => break,
            }
        }

        let board = parse_anfield(&raw_rows, cols, rows, player);

        // Read Piece header: "Piece <rows> <cols>:"
        let piece_header = match lines_iter.next() {
            Some(l) => l,
            None => break,
        };
        let (p_rows, p_cols) = parser::parse_piece_dimensions(&piece_header);

        let mut piece_rows: Vec<String> = Vec::with_capacity(p_rows);
        for _ in 0..p_rows {
            match lines_iter.next() {
                Some(l) => piece_rows.push(l),
                None => break,
            }
        }

        let piece = parse_piece(&piece_rows, p_rows, p_cols);

        let (x, y) = find_best_placement(&board, &piece, cols, rows);

        writeln!(out, "{} {}", x, y).expect("Failed to write output");
        out.flush().expect("Failed to flush");
    }
}
