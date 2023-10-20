
pub fn get_possibilities(s: &[[i32; 9]; 9], i: usize, j: usize) -> [bool; 9] {
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
    return poss;
}
