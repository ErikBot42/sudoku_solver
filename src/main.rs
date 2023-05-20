fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let expected = vec![
        vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
        vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
        vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
        vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
        vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
        vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
        vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
        vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
        vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
    ];
    board_print(&board);
    solve_sudoku(&mut board);
    println!("current:");
    board_print(&board);
    println!("expected:");
    board_print(&expected);
    assert_eq!(expected, board);
    fn board_print<T: std::fmt::Debug>(board: &Vec<Vec<T>>) {
        println!("Board:");
        for row in board {
            println!("{row:?}");
        }
        println!();
    }
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    fn reverse_parse_board_char(c: &u8) -> char {
        match c {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => unreachable!(),
        }
    }

    fn parse_board_char(c: &char) -> Option<u8> {
        match c {
            '1' => Some(1),
            '2' => Some(2),
            '3' => Some(3),
            '4' => Some(4),
            '5' => Some(5),
            '6' => Some(6),
            '7' => Some(7),
            '8' => Some(8),
            '9' => Some(9),
            '.' => None,
            _ => unreachable!(),
        }
    }

    // no heap, only stack.
    const ALL_POSSIBILITES: u16 = 0b11111_1111_0;
    #[derive(Debug, Clone)]
    struct BoardState {
        rows: [u16; 9],
        cols: [u16; 9],
        groups: [u16; 9],
        state: [u8; 81],

        row_left: [u8; 9],
        col_left: [u8; 9],
        group_left: [u8; 9],

        state_left: u8,
    }
    impl BoardState {
        fn pretty_print(&self) {
            println!("board:");
            for row in self.state.chunks(9) {
                println!("{row:?}");
            }
            println!();
        }
        fn solve(state: BoardState) -> Option<BoardState> {
            let now = std::time::Instant::now();
            let mut counter = 0;
            let rval = Self::solve_rec(&state, 0, &mut counter);
            println!("elapsed: {:?}, with {counter:?} iterations", now.elapsed());
            rval
        }
        fn solve_rec(
            state: &BoardState,
            position: usize,
            counter: &mut usize,
        ) -> Option<BoardState> {
            *counter += 1;
            if position == 81 {
                Some(state.clone())
            } else if state.state[position] != 0 {
                Self::solve_rec(state, position + 1, counter)
            } else {
                let (row, col, group) = Self::row_col_group(position);

                let mut possible: u16 = state.rows[row] & state.cols[col] & state.groups[group];

                while possible != 0 {
                    let val: u8 = possible.trailing_zeros() as u8;
                    possible &= !(1 << val);
                    let mut new_state = state.clone();
                    //new_state.set_cell(position, val);
                    if !new_state.set_cell(position, val) {
                        continue;
                    }
                    if let Some(solved_state) = Self::solve_rec(&new_state, position + 1, counter) {
                        return Some(solved_state);
                    }
                }
                None
            }
        }

        fn row_col_group(position: usize) -> (usize, usize, usize) {
            let row = position % 9;
            let col = position / 9;
            let group = row / 3 + (col / 3) * 3;
            (row, col, group)
        }

        /// PRE: only set valid possibility
        ///
        /// Set cell value, return whether board could be solvable
        fn set_cell(&mut self, position: usize, val: u8) -> bool {
            assert_eq!(self.state[position], 0);
            let val_mask: u16 = !(1 << (val as u16));
            let (row, col, group) = Self::row_col_group(position);

            self.rows[row] &= val_mask;
            self.cols[col] &= val_mask;
            self.groups[group] &= val_mask;
            self.state[position] = val;

            self.row_left[row] -= 1;
            self.col_left[col] -= 1;
            self.group_left[group] -= 1;
            self.state_left -= 1;

            self.rows[row].count_ones() as u8 >= self.row_left[row]
                && self.cols[col].count_ones() as u8 >= self.col_left[col]
                && self.groups[group].count_ones() as u8 >= self.group_left[group]
        }
        
        fn from_board(board: &Vec<Vec<Option<u8>>>) -> Self {
            let mut state = Self::new();
            for row in 0..9 {
                for col in 0..9 {
                    if let Some(s) = board[row][col] {
                        assert!(state.set_cell(row * 9 + col, s));
                    }
                }
            }
            state
        }
        fn new() -> Self {
            Self {
                rows: [ALL_POSSIBILITES; 9],
                cols: [ALL_POSSIBILITES; 9],
                groups: [ALL_POSSIBILITES; 9],
                state: [0; 81],

                row_left: [9; 9],
                col_left: [9; 9],
                group_left: [9; 9],
                state_left: 81,
            }
        }
    }

    let parsed_board: Vec<Vec<Option<u8>>> = board
        .iter()
        .map(|b| b.iter().map(parse_board_char).collect())
        .collect();
    let output_board = board;
    let board = parsed_board;

    let state = BoardState::from_board(&board);

    let solved_state = BoardState::solve(state).unwrap();
    solved_state.pretty_print();

    *output_board = solved_state
        .state
        .chunks(9)
        .map(|a| a.iter().map(reverse_parse_board_char).collect())
        .collect();
}
