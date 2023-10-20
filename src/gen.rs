mod shuffle;
mod solve;
mod solver1;
mod solver2;
use shuffle::shuffle_array;
pub struct Sudoku {
    pub problem: [[i32; 9]; 9],
    pub solution: [[i32; 9]; 9],
    pub score: usize,
}
pub fn solve_sudoku_simple(s: &mut [[i32; 9]; 9]) -> bool {
    fn solve_step(s: &mut [[i32; 9]; 9], i: usize, j: usize) -> bool {
        if i > 8 {
            return true;
        }
        if s[i][j] != 0 {
            if j == 8 {
                return solve_step(s, i + 1, 0);
            } else {
                return solve_step(s, i, j + 1);
            }
        }
        // check possibilities
        let mut poss: [bool; 9] = solve::get_possibilities(s, i, j);
        // check in square
        let basei = (i / 3) * 3;
        let basej = (j / 3) * 3;
        for p in 0..3 {
            for q in 0..3 {
                let f = s[basei + p][basej + q];
                if f != 0 {
                    poss[(f - 1) as usize] = false;
                }
            }
        }
        let mut numbers: [usize; 9] = core::array::from_fn(|i| i);
        shuffle_array(&mut numbers);
        // backtracking
        for p in numbers {
            if poss[p] {
                s[i][j] = (p + 1) as i32;
                let (ni, nj) = if j == 8 { (i + 1, 0) } else { (i, j + 1) };
                if solve_step(s, ni, nj) {
                    return true;
                }
                s[i][j] = 0;
            }
        }
        return false;
    }
    return solve_step(s, 0, 0);
}
fn delete_fields(s: &mut [[i32; 9]; 9], level: u8) -> usize {
    let mut score = 1;
    'delloop: loop {
        let mut ind1: [usize; 9] = core::array::from_fn(|i| i);
        let mut ind2: [usize; 9] = core::array::from_fn(|i| i);
        shuffle_array(&mut ind1);
        shuffle_array(&mut ind2);
        for i in ind1 {
            for j in ind2 {
                if s[i][j] == 0 {
                    continue;
                }
                let prev = s[i][j];
                s[i][j] = 0;
                match level {
                    0 => {
                        if solver1::solve_logic1(s) {
                            continue 'delloop;
                        }
                    }
                    1 => {
                        let local_score = solver2::solve_logic2(s);
                        if local_score > 0 {
                            score = local_score;
                            continue 'delloop;
                        }
                    }
                    _ => {}
                }
                s[i][j] = prev;
            }
        }
        break;
    }
    return score;
}
pub fn generate(level: u8) -> Sudoku {
    let mut empty = [[0; 9]; 9];
    solve_sudoku_simple(&mut empty);
    let mut problem = [[0; 9]; 9];

    for i in 0..9 {
        for j in 0..9 {
            problem[i][j] = empty[i][j];
        }
    }
    let score = delete_fields(&mut problem, level);
    let sudoku = Sudoku {
        problem,
        solution: empty,
        score,
    };
    return sudoku;
}
