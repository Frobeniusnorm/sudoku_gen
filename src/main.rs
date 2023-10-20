pub mod gen;
fn main() {
    let mut s = gen::generate();
    for _j in 0..9 {
        print!("*-");
    }
    print!("*\n");
    for i in 0..9 {
        for j in 0..9 {
            if s[i][j] == 0 {
                print!("| ");
            } else {
                print!("|{:}", s[i][j]);
            }
        }
        print!("|\n");
        for _j in 0..9 {
            print!("*-");
        }
        print!("*\n");
    }
}
