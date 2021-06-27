use std::io;

mod utils;
mod intro;
mod sorting_searching;
//mod dynamic_programming;


fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = utils::Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());
    sorting_searching::solve_distinctnumbers(&mut scan, &mut out);
}
