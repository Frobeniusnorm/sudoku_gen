// TODO: forcing chains
fn influenced_cells(s: &[[i32; 9]; 9], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut influenced: Vec<(usize, usize)> = vec![];
    for k in 0..9 {
        if s[i][k] == 0 && k != j {
            influenced.push((i, k));
        }
        if s[k][j] == 0 && k != i {
            influenced.push((k, j));
        }
    }
    let basei = (i / 3) * 3;
    let basej = (j / 3) * 3;
    for p in 0..3 {
        for q in 0..3 {
            if s[basei + p][basej + q] == 0 && (basei + p != i || basej + q != j) {
                influenced.push((basei + p, basej + q));
            }
        }
    }
    return influenced;
}
use super::solve::get_possibilities;
use super::solver2::distinct_possibility;
pub fn solve_logic3(s: &mut [[i32; 9]; 9]) -> bool {
    // level 3
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                let poss = get_possibilities(s, i, j);
                {
                    let mut poss_count = 0;
                    for p in poss {
                        if p {
                            poss_count += 1;
                        }
                    }
                    if poss_count > 4 {
                        continue; // avoid too long chains
                    }
                }
                // get influenced empty cells
                let influenced_orig = influenced_cells(s, i, j);
                let influenced = &influenced_orig;
                // certain values for cells per pass
                let mut certain: Vec<Vec<i32>> = Vec::with_capacity(influenced.len());
                for _inf in influenced {
                    certain.push(Vec::new());
                }
                for p in 0..9 {
                    if poss[p] {
                        let mut forposs: Vec<i32> = Vec::with_capacity(influenced.len());
                        for _inf in influenced {
                            forposs.push(-1);
                        }
                        // write cell value
                        s[i][j] = (p + 1) as i32;
                        // iterate for influenced fields, but stop to avoid too long chains
                        for _ in 0..3 {
                            let mut changed = false;
                            for inf in 0..influenced.len() {
                                if forposs[inf] == -1 {
                                    // check possibilities and check if they were the same
                                    let poss_inf =
                                        get_possibilities(s, influenced[inf].0, influenced[inf].1);
                                    let mut poss_val = 0;
                                    let mut poss_count = 0;
                                    for p_i in 0..9 {
                                        if poss_inf[p_i] {
                                            poss_val = p_i;
                                            poss_count += 1;
                                        }
                                    }
                                    if poss_count == 1 {
                                        changed = true;
                                        forposs[inf] = poss_val as i32;
                                        s[influenced[inf].0][influenced[inf].1] =
                                            (poss_val + 1) as i32;
                                    }
                                }
                            }
                            if !changed {
                                break;
                            }
                        }
                        s[i][j] = 0;
                        for inf in influenced {
                            s[inf.0][inf.1] = 0;
                        }
                        // add forposs in certain
                        for inf in 0..influenced.len() {
                            certain[inf].push(forposs[inf]);
                        }
                    }
                }
                // those were certain is for all possibilities the same may be set
                for inf in 0..influenced.len() {
                    let mut same = true;
                    // check if for all iterations same
                    for it in 1..certain[inf].len() {
                        if certain[inf][it - 1] != certain[inf][it] {
                            same = false;
                            break;
                        }
                    }
                    if same && !certain[inf].is_empty() && certain[inf][0] >= 0 {
                        s[influenced[inf].0][influenced[inf].1] = certain[inf][0] + 1;
                        let rekres = solve_logic3(s);
                        s[influenced[inf].0][influenced[inf].1] = 0;
                        return rekres;
                    }
                }
            }
        }
    }
    // did not work -> level2 logic
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                let poss = get_possibilities(s, i, j);
                let (v, difficulty) = distinct_possibility(&s, &poss, i, j);
                if v >= 0 {
                    s[i][j] = v + 1;
                    let rekres = solve_logic3(s);
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
                    let rekres = solve_logic3(s);
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
    // level 3
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                let poss = get_possibilities(s, i, j);
                // get influenced empty cells
                let influenced_orig = influenced_cells(s, i, j);
                let influenced = &influenced_orig;
                // certain values for cells per pass
                let mut certain: Vec<Vec<i32>> = Vec::with_capacity(influenced.len());
                for _inf in influenced {
                    certain.push(Vec::new());
                }
                let mut difficulty = 0;
                for p in 0..9 {
                    if poss[p] {
                        let mut forposs: Vec<i32> = Vec::with_capacity(influenced.len());
                        for _inf in influenced {
                            forposs.push(-1);
                        }
                        // write cell value
                        s[i][j] = (p + 1) as i32;
                        // iterate for influenced fields, but stop to avoid too long chains
                        let mut diff = 1;
                        loop {
                            let mut changed = false;
                            for inf in 0..influenced.len() {
                                if forposs[inf] == -1 {
                                    // check possibilities and check if they were the same
                                    let poss_inf =
                                        get_possibilities(s, influenced[inf].0, influenced[inf].1);
                                    let mut poss_val = 0;
                                    let mut poss_count = 0;
                                    for p_i in 0..9 {
                                        if poss_inf[p_i] {
                                            poss_val = p_i;
                                            poss_count += 1;
                                        }
                                    }
                                    if poss_count == 1 {
                                        changed = true;
                                        forposs[inf] = poss_val as i32;
                                        s[influenced[inf].0][influenced[inf].1] =
                                            (poss_val + 1) as i32;
                                    }
                                }
                            }
                            if !changed {
                                break;
                            }
                            diff += 1;
                        }
                        difficulty += diff * poss.len() * 7;
                        s[i][j] = 0;
                        for inf in influenced {
                            s[inf.0][inf.1] = 0;
                        }
                        // add forposs in certain
                        for inf in 0..influenced.len() {
                            certain[inf].push(forposs[inf]);
                        }
                    }
                }
                // those were certain is for all possibilities the same may be set
                for inf in 0..influenced.len() {
                    let mut same = true;
                    // check if for all iterations same
                    for it in 1..certain[inf].len() {
                        if certain[inf][it - 1] != certain[inf][it] {
                            same = false;
                            break;
                        }
                    }
                    if same && !certain[inf].is_empty() && certain[inf][0] >= 0 {
                        println!("logic3");
                        s[influenced[inf].0][influenced[inf].1] = certain[inf][0] + 1;
                        let rekres = score(s);
                        s[influenced[inf].0][influenced[inf].1] = 0;
                        if rekres > 0 {
                            return rekres + difficulty;
                        } else {
                            return 0;
                        }
                    }
                }
            }
        }
    }
    return 0;
}
