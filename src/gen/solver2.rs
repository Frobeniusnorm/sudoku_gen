use super::solve::get_possibilities;

fn distinct_possibility(
    s: &[[i32; 9]; 9],
    possibilities: &[bool; 9],
    i: usize,
    j: usize,
) -> (i32, usize) {
    let distinct1 = &mut [false; 9];
    let distinct2 = &mut [false; 9];
    let distinct3 = &mut [false; 9];
    for k in 0..9 {
        distinct1[k] = possibilities[k];
        distinct2[k] = possibilities[k];
        distinct3[k] = possibilities[k];
    }
    let mut difficulty1 = 0;
    let mut difficulty2 = 0;
    let mut difficulty3 = 0;
    // row and column
    for k in 0..9 {
        if s[i][k] == 0 && k != j {
            let kposs = get_possibilities(s, i, k);
            difficulty1 += kposs.len();
            for p in 0..9 {
                if kposs[p] {
                    distinct1[p] = false;
                }
            }
        }
        if s[k][j] == 0 && k != i {
            let kposs = get_possibilities(s, k, j);
            difficulty2 += kposs.len();
            for p in 0..9 {
                if kposs[p] {
                    distinct2[p] = false;
                }
            }
        }
    }
    // block
    let basei = (i / 3) * 3;
    let basej = (j / 3) * 3;
    for p in 0..3 {
        for q in 0..3 {
            if s[basei + p][basej + q] == 0 && (basei + p != i || basej + q != j) {
                let kposs = get_possibilities(s, basei + p, basej + q);
                difficulty3 += kposs.len();
                for k in 0..9 {
                    if kposs[k] {
                        distinct3[k] = false;
                    }
                }
            }
        }
    }
    // get distinct values
    let mut count = 0;
    let mut value1 = -1;
    let mut value2 = -1;
    let mut value3 = -1;
    let mut difficulty = 0;
    for k in 0..9 {
        if distinct1[k] {
            count += 1;
            value1 = k as i32;
            difficulty = difficulty1;
        }
        if distinct2[k] {
            count += 1;
            value2 = k as i32;
            difficulty = difficulty2;
        }
        if distinct3[k] {
            count += 1;
            value3 = k as i32;
            difficulty = difficulty3;
        }
    }
    let mut value = value1;
    for v in [value1, value2, value3] {
        if count == 1 && v != -1 {
            value = v;
            break;
        } else {
            if v != -1 {
                if value == -1 || v == value {
                    value = v;
                } else {
                    value = -1;
                    break;
                }
            }
        }
    }
    if value != -1 {
        return (value, difficulty);
    }
    return (-1, 0);
}

// returns score
pub fn solve_logic2(s: &mut [[i32; 9]; 9]) -> bool {
    /* solve_logic1 + per group (box, row, column) for empty fields with multiple possibilities
     * test if one value is only possible or one field, then insert. */
    // first try advanced logic
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                let poss = get_possibilities(s, i, j);
                let (v, difficulty) = distinct_possibility(&s, &poss, i, j);
                if v >= 0 {
                    s[i][j] = v + 1;
                    let rekres = solve_logic2(s);
                    s[i][j] = 0;
                    return rekres;
                }
            }
        }
    }
    // did not work -> level1 logic
    let mut was_zero = false;
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                was_zero = true;
                let poss = get_possibilities(s, i, j);
                let mut count = 0;
                let mut last = 0;
                for k in 0..9 {
                    if poss[k] {
                        count += 1;
                        last = k;
                    }
                }
                if count == 0 {
                    return true;
                }
                if count == 1 {
                    s[i][j] = (last + 1) as i32;
                    let rekres = solve_logic2(s);
                    s[i][j] = 0;
                    return rekres;
                }
            }
        }
    }
    return !was_zero;
}

pub fn score(s: &mut [[i32; 9]; 9]) -> usize {
    let mut was_zero = false;
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                was_zero = true;
                let poss = get_possibilities(s, i, j);
                let mut count = 0;
                let mut last = 0;
                for k in 0..9 {
                    if poss[k] {
                        count += 1;
                        last = k;
                    }
                }
                if count == 0 {
                    return 1;
                }
                if count == 1 {
                    s[i][j] = (last + 1) as i32;
                    let rekres = score(s);
                    s[i][j] = 0;
                    return rekres;
                }
            }
        }
    }
    if !was_zero {
        return 1;
    }
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                let poss = get_possibilities(s, i, j);
                let (v, difficulty) = distinct_possibility(&s, &poss, i, j);
                if v >= 0 {
                    s[i][j] = v + 1;
                    let rekres = score(s);
                    s[i][j] = 0;
                    if rekres > 0 {
                        return rekres + difficulty;
                    } else {
                        return 0;
                    }
                }
            }
        }
    }
    return 0;
}

