mod draw;
pub mod gen;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="Frobeniusnorm", version="0.0.1", about, long_about = None)]
struct Args {
    // reasoning level to apply (0, 1, 2)
    #[arg(short, long, default_value_t = 2)]
    reasoning: u8,

    // print solution
    #[arg(short, long, default_value_t = true)]
    solution: bool,

    // generate image (disables per default printing on stdout, see print)
    #[arg(short, long, default_value_t = false)]
    image: bool,

    // print sodoku on stdout additional to image generation
    #[arg(short, long, default_value_t = false)]
    print: bool,
}
fn print_sudoku(s: &[[i32; 9]; 9]) {
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
fn main() {
   let args = Args::parse();
   let s = gen::generate(args.reasoning);
   println!("score: {}", s.score);
   if args.print || !args.image {
       print_sudoku(&s.problem);
   }
   if (args.print || !args.image) && args.solution {
       print_sudoku(&s.solution);
   }
   if args.image {
       draw::draw_sudoku(&s, args.solution);
   }
}
