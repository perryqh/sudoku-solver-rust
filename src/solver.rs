pub mod solver {
    const SOLVED_CELL: [usize; 2] = [10, 10];
    const EMPTY_CELL: i8 = 0;

    pub fn valid(board: [[i8; 9]; 9], row: usize, column: usize, guess: i8) -> bool {
        for x in 0..9 {
            if board[row][x] == guess || board[x][column] == guess {
                return false;
            }
        }
        valid_three_by_three(board, row, column, guess)
    }

    fn valid_three_by_three(board: [[i8; 9]; 9], row: usize, column: usize, guess: i8) -> bool {
        let x_index: usize = row / 3 * 3;
        let y_index: usize = column / 3 * 3;

        for x in 0..3 {
            for y in 0..3 {
                if board[x_index + x][y_index + y] == guess {
                    return false;
                }
            }
        }
        true
    }

    fn is_solved(board: [[i8; 9]; 9]) -> bool {
        next_empty_cell(board) == SOLVED_CELL
    }

    fn next_empty_cell(board: [[i8; 9]; 9]) -> [usize; 2] {
        for row in 0..9 {
            for column in 0..9 {
                if board[row][column] == EMPTY_CELL {
                    return [row, column];
                }
            }
        }
        SOLVED_CELL
    }

    fn guesses(board: [[i8; 9]; 9], row: usize, column: usize) -> Vec<i8> {
        let mut result = vec![];
        for guess in 1..10 {
            if valid(board, row, column, guess) {
                result.push(guess);
            }
        }
        result
    }

    pub fn solve(board: [[i8; 9]; 9]) -> [[i8; 9]; 9] {
        println!("Current board is: {:?}", board);
        let mut result = board;

        let empty_cell: [usize; 2] = next_empty_cell(board);

        if is_solved(board) {
            return result;
        }

        println!("Found empty cell: {:?}", empty_cell);

        let row: usize = empty_cell[0];
        let column: usize = empty_cell[1];

        for guess in guesses(board, row, column) {
            result[row][column] = guess;
            result = solve(result);
            if is_solved(result) {
                return result;
            }
        }
        board
    }
}

fn test_board() -> [[i8; 9]; 9] {
    [
        [1, 7, 4, 0, 9, 0, 6, 0, 0],
        [0, 0, 0, 0, 3, 8, 1, 5, 7],
        [5, 3, 0, 7, 0, 1, 0, 0, 4],
        [0, 0, 7, 3, 4, 9, 8, 0, 0],
        [8, 4, 0, 5, 0, 0, 3, 6, 0],
        [3, 0, 5, 0, 0, 6, 4, 7, 0],
        [2, 8, 6, 9, 0, 0, 0, 0, 1],
        [0, 0, 0, 6, 2, 7, 0, 3, 8],
        [0, 5, 3, 0, 8, 0, 0, 9, 6],
    ]
}

fn almost_solved_test_board() -> [[i8; 9]; 9] {
    [
        [1, 7, 4, 0, 9, 5, 6, 8, 3],
        [9, 6, 2, 4, 3, 8, 1, 5, 7],
        [5, 3, 8, 7, 6, 1, 9, 2, 4],
        [6, 2, 7, 3, 4, 9, 8, 1, 5],
        [8, 4, 1, 5, 7, 2, 3, 6, 9],
        [3, 9, 5, 0, 0, 6, 4, 7, 2],
        [2, 8, 6, 0, 0, 3, 7, 4, 1],
        [4, 1, 9, 0, 0, 0, 5, 3, 8],
        [7, 5, 3, 1, 8, 4, 2, 9, 6],
    ]
}

#[test]
fn returns_invalid_for_dupe_in_three_by_three() {
    assert!(!solver::valid(test_board(), 1, 0, 4))
}

#[test]
fn returns_invalid_for_dupe_in_column() {
    assert!(!solver::valid(test_board(), 1, 0, 1))
}

#[test]
fn returns_invalid_for_dupe_in_column_and_three_by_three() {
    assert!(!solver::valid(test_board(), 1, 1, 4))
}

#[test]
fn returns_invalid_for_another_dupe_in_column_and_three_by_three() {
    assert!(!solver::valid(test_board(), 1, 2, 4))
}

#[test]
fn returns_invalid_for_dupe_in_row() {
    assert!(!solver::valid(test_board(), 0, 3, 4))
}

#[test]
fn returns_invalid_for_dupe_in_column_and_row() {
    assert!(!solver::valid(test_board(), 0, 3, 3))
}

#[test]
fn returns_invalid_for_dupe_in_column_and_last_three_by_three() {
    assert!(!solver::valid(test_board(), 6, 6, 3))
}

#[test]
fn returns_invalid_for_8_6_1() {
    assert!(!solver::valid(test_board(), 8, 6, 1))
}

#[test]
fn returns_valid_for_0_3_2() {
    assert!(solver::valid(test_board(), 0, 3, 2))
}

#[test]
fn returns_solved_board() {
    assert_eq!(
        solver::solve(test_board()),
        [
            [1, 7, 4, 2, 9, 5, 6, 8, 3],
            [9, 6, 2, 4, 3, 8, 1, 5, 7],
            [5, 3, 8, 7, 6, 1, 9, 2, 4],
            [6, 2, 7, 3, 4, 9, 8, 1, 5],
            [8, 4, 1, 5, 7, 2, 3, 6, 9],
            [3, 9, 5, 8, 1, 6, 4, 7, 2],
            [2, 8, 6, 9, 5, 3, 7, 4, 1],
            [4, 1, 9, 6, 2, 7, 5, 3, 8],
            [7, 5, 3, 1, 8, 4, 2, 9, 6]
        ]
    )
}

#[test]
fn returns_board_easy() {
    assert_eq!(
        solver::solve(almost_solved_test_board()),
        [
            [1, 7, 4, 2, 9, 5, 6, 8, 3],
            [9, 6, 2, 4, 3, 8, 1, 5, 7],
            [5, 3, 8, 7, 6, 1, 9, 2, 4],
            [6, 2, 7, 3, 4, 9, 8, 1, 5],
            [8, 4, 1, 5, 7, 2, 3, 6, 9],
            [3, 9, 5, 8, 1, 6, 4, 7, 2],
            [2, 8, 6, 9, 5, 3, 7, 4, 1],
            [4, 1, 9, 6, 2, 7, 5, 3, 8],
            [7, 5, 3, 1, 8, 4, 2, 9, 6]
        ]
    )
}