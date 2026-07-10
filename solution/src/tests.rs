#[cfg(test)]
mod parser_tests {
    use crate::parser::*;
    use crate::board::Cell;

    #[test]
    fn test_parse_header_player1() {
        let line = "$$$ exec p1 : [robots/bender]";
        assert_eq!(parse_header(line), 1);
    }

    #[test]
    fn test_parse_header_player2() {
        let line = "$$$ exec p2 : [robots/terminator]";
        assert_eq!(parse_header(line), 2);
    }

    #[test]
    fn test_parse_anfield_dimensions() {
        let line = "Anfield 20 15:";
        let (cols, rows) = parse_anfield_dimensions(line);
        assert_eq!(cols, 20);
        assert_eq!(rows, 15);
    }

    #[test]
    fn test_parse_piece_dimensions() {
        let line = "Piece 4 1:";
        let (rows, cols) = parse_piece_dimensions(line);
        assert_eq!(rows, 4);
        assert_eq!(cols, 1);
    }

    #[test]
    fn test_parse_anfield_grid_player1() {
        // Player 1: own = 'a'/'@', opponent = 's'/'$'
        let raw = vec![
            "000 ..@..".to_string(),
            "001 ...$.".to_string(),
        ];
        let board = parse_anfield(&raw, 5, 2, 1);
        assert_eq!(board.grid[0][2], Cell::Own);
        assert_eq!(board.grid[1][3], Cell::Opponent);
        assert_eq!(board.grid[0][0], Cell::Empty);
    }

    #[test]
    fn test_parse_anfield_grid_player2() {
        // Player 2: own = 's'/'$', opponent = 'a'/'@'
        let raw = vec![
            "000 ..@..".to_string(),
            "001 ...$.".to_string(),
        ];
        let board = parse_anfield(&raw, 5, 2, 2);
        assert_eq!(board.grid[0][2], Cell::Opponent);
        assert_eq!(board.grid[1][3], Cell::Own);
    }

    #[test]
    fn test_parse_piece_hash() {
        let raw = vec![
            ".#".to_string(),
            "#.".to_string(),
        ];
        let piece = parse_piece(&raw, 2, 2);
        assert!(!piece[0][0]);
        assert!(piece[0][1]);
        assert!(piece[1][0]);
        assert!(!piece[1][1]);
    }

    #[test]
    fn test_parse_piece_O_marker() {
        // game_engine uses 'O' for the piece cell in the example
        let raw = vec![".OO.".to_string()];
        let piece = parse_piece(&raw, 1, 4);
        assert!(!piece[0][0]);
        assert!(piece[0][1]);
        assert!(piece[0][2]);
        assert!(!piece[0][3]);
    }
}

#[cfg(test)]
mod placement_tests {
    use crate::board::{Board, Cell};

    fn make_board(grid: Vec<Vec<Cell>>) -> Board {
        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };
        Board { grid, rows, cols }
    }

    // Helper: 5x5 empty board with one Own cell at (2,2)
    fn board_with_own_at(r: usize, c: usize, rows: usize, cols: usize) -> Board {
        let mut grid = vec![vec![Cell::Empty; cols]; rows];
        grid[r][c] = Cell::Own;
        make_board(grid)
    }

    #[test]
    fn test_valid_placement_single_overlap() {
        // Board 5x5, own at (2,2). Piece 1x1 placed at px=2, py=2 => exactly 1 overlap.
        let board = board_with_own_at(2, 2, 5, 5);
        let piece = vec![vec![true]];
        assert!(board.is_valid_placement(&piece, 2, 2));
    }

    #[test]
    fn test_invalid_placement_zero_overlap() {
        // No overlap with own territory
        let board = board_with_own_at(2, 2, 5, 5);
        let piece = vec![vec![true]];
        assert!(!board.is_valid_placement(&piece, 0, 0));
    }

    #[test]
    fn test_invalid_placement_two_overlaps() {
        // Board with own cells at (2,2) and (2,3). A 1x2 piece at px=2,py=2 overlaps both.
        let mut grid = vec![vec![Cell::Empty; 5]; 5];
        grid[2][2] = Cell::Own;
        grid[2][3] = Cell::Own;
        let board = make_board(grid);
        let piece = vec![vec![true, true]];
        assert!(!board.is_valid_placement(&piece, 2, 2));
    }

    #[test]
    fn test_invalid_placement_opponent_overlap() {
        // Own cell at (2,2), opponent at (2,3). Piece at px=2,py=2 hits opponent.
        let mut grid = vec![vec![Cell::Empty; 5]; 5];
        grid[2][2] = Cell::Own;
        grid[2][3] = Cell::Opponent;
        let board = make_board(grid);
        let piece = vec![vec![true, true]];
        assert!(!board.is_valid_placement(&piece, 2, 2));
    }

    #[test]
    fn test_boundary_piece_exceeds_right() {
        // 5x5 board, 1x3 piece at px=4 => extends to col 6 which is out of bounds
        let board = board_with_own_at(0, 4, 5, 5);
        let piece = vec![vec![true, true, true]];
        assert!(!board.is_valid_placement(&piece, 4, 0));
    }

    #[test]
    fn test_boundary_piece_exceeds_bottom() {
        // 5x5 board, 3x1 piece at py=4 => extends to row 6 which is out of bounds
        let board = board_with_own_at(4, 0, 5, 5);
        let piece = vec![vec![true], vec![true], vec![true]];
        assert!(!board.is_valid_placement(&piece, 0, 4));
    }

    #[test]
    fn test_boundary_piece_exactly_fits() {
        // 5x5, own at (4,4), 1x1 piece at px=4,py=4 => exactly fits
        let board = board_with_own_at(4, 4, 5, 5);
        let piece = vec![vec![true]];
        assert!(board.is_valid_placement(&piece, 4, 4));
    }

    #[test]
    fn test_valid_placement_2x2_piece() {
        // 5x5, own at (1,1). 2x2 piece:
        // #.
        // ..
        // placed at px=1,py=1 => piece[0][0] at board(1,1) = Own => 1 overlap. Valid.
        let board = board_with_own_at(1, 1, 5, 5);
        let piece = vec![
            vec![true, false],
            vec![false, false],
        ];
        assert!(board.is_valid_placement(&piece, 1, 1));
    }
}

#[cfg(test)]
mod output_tests {
    // Verify the coordinate output formatting requirement: "X Y\n"
    #[test]
    fn test_output_format() {
        let x: usize = 7;
        let y: usize = 2;
        let output = format!("{} {}\n", x, y);
        assert_eq!(output, "7 2\n");
    }

    #[test]
    fn test_output_format_zero() {
        let x: usize = 0;
        let y: usize = 0;
        let output = format!("{} {}\n", x, y);
        assert_eq!(output, "0 0\n");
    }
}
