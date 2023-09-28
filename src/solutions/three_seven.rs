fn _print_sudoku(board: &Vec<Vec<char>>) {
    for i in 0..9 {
        println!("{:?}", board[i]);
    }
}

fn _is_valid(board: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let mut v_i = vec![false;9];
    let mut v_j = vec![false;9];
    let mut v_s = vec![false;9];
    for x in 0..9 {
        // check horizontal
        if let Some(n) = board[i][x].to_digit(10) {
            let n_1 = n as usize - 1;
            if v_i[n_1] {
                return false;
            }
            v_i[n_1] = true;
        }
        // check vertical
        if let Some(n) = board[x][j].to_digit(10) {
            let n_1 = n as usize - 1;
            if v_j[n_1] {
                return false;
            }
            v_j[n_1] = true;
        }
    }
    // check box
    for a in 0..3 {
        for b in 0..3 {
            let k = a + i / 3 * 3;
            let l = b + j / 3 * 3;
            if let Some(n) = board[k][l].to_digit(10) {
                let n_1 = n as usize - 1;
                if v_s[n_1] {
                    return false;
                }
                v_s[n_1] = true;
            }
        }
    }
    true
}

fn _solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    //println!("");
    //_print_sudoku(&board);
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == '.' {
                for k in vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                    board[i][j] = k;
                    if _is_valid(board, i, j) && _solve_sudoku(board) {
                        return true;
                    }
                }
                // no digit fits, backtrack to the previous cell
                board[i][j] = '.';
                return false;
            }
        }
    }
    true
}

pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    _solve_sudoku(board);
}

#[test]
pub fn test_solve_sudoku() {
    let mut b: Vec<Vec<char>> = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];
    let b_s: Vec<Vec<char>> = vec![
        vec!['5','3','4','6','7','8','9','1','2'],
        vec!['6','7','2','1','9','5','3','4','8'],
        vec!['1','9','8','3','4','2','5','6','7'],
        vec!['8','5','9','7','6','1','4','2','3'],
        vec!['4','2','6','8','5','3','7','9','1'],
        vec!['7','1','3','9','2','4','8','5','6'],
        vec!['9','6','1','5','3','7','2','8','4'],
        vec!['2','8','7','4','1','9','6','3','5'],
        vec!['3','4','5','2','8','6','1','7','9']
    ];
    solve_sudoku(&mut b);
    assert_eq!(b_s, b);
}