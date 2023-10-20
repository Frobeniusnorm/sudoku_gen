use rand::Rng;
pub fn shuffle_array(p: &mut [usize; 9]) {
    let mut rng = rand::thread_rng();
    for i in 0..9 {
        let other: usize = rng.gen();
        let prev = p[i];
        p[i] = p[other % 9];
        p[other % 9] = prev;
    }
}
