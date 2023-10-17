use rand::Rng;

fn solve_sudoku(s: &mut [[i32; 9]; 9]) -> bool {
    fn shuffle_array(p: &mut[usize; 9]) {
        let mut rng = rand::thread_rng();
        for i in 0..9 {
            let other : usize = rng.gen();
            let prev = p[i];
            p[i] = p[other % 9];
            p[other % 9] = prev;
        }
    }
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
        let mut poss: [bool; 9] = [true; 9];
        for p in 0..9 {
            // check in row
            if s[i][p] != 0 {
                poss[(s[i][p] - 1) as usize] = false;
            }
            // check in col
            if s[p][j] != 0 {
                poss[(s[p][j] - 1) as usize] = false;
            }
        }
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
        let mut numbers : [usize; 9] = core::array::from_fn(|i| i);
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


fn main() {
    let mut s = 
            [[4, 1, 0, 0, 6, 5, 0, 0, 7],
             [0, 0, 6, 0, 0, 7, 4, 8, 0],
             [2, 0, 7, 4, 9, 0, 0, 0, 6],
             [0, 6, 0, 0, 7, 0, 1, 0, 0],
             [3, 0, 1, 5, 0, 0, 0, 7, 2],
             [0, 9, 0, 0, 4, 2, 3, 0, 8],
             [1, 0, 8, 6, 0, 0, 0, 2, 0],
             [0, 2, 0, 0, 1, 8, 6, 4, 0],
             [6, 0, 0, 3, 0, 0, 0, 1, 0]];
    if solve_sudoku(&mut s) {
        println!("Solved: {:?}", s);
    } else {
        println!("Not Solveable!");
    }
}
