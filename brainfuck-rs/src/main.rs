mod lib;

use lib::BrainFuck;

fn main() {
    let program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let mut compiler = BrainFuck::new();
    compiler.compile(program);
}
