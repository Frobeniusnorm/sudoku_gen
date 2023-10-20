use super::solve::get_possibilities;
pub fn solve_logic1(s: &mut [[i32; 9]; 9]) -> bool {
    let mut was_zero = false;
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                was_zero = true;
                let poss = get_possibilities(*s, i, j);
                let mut count = 0;
                let mut last = 0;
                for k in 0..9 {
                    if poss[k] {
                        count += 1;
                        last = k;
                    }
                }
                if count == 0 {
                    return false;
                }
                if count == 1 {
                    s[i][j] = (last + 1) as i32;
                    let rekres = solve_logic1(s);
                    s[i][j] = 0;
                    return rekres;
                }
            }
        }
    } 
    return !was_zero; 
}
