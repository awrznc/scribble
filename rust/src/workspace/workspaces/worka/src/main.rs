use workb::*;

#[derive(Work)]
struct A {}

impl A {
    pub fn return_zero(&self) -> usize {
        0
    }
}

#[derive(Debug, Work)]
struct B {
    b: u8,
    bb: u16,
    bbb: u32,
    bbbb: u64,
}

#[print_args(aaa)]
fn main() {
    println!("{:?}", return_input!(42));
    // => 42

    let a = A {};
    a.return_zero();
    a.print_work();
    a.print_contents();

    let b = B {
        b: 0,
        bb: 1,
        bbb: 2,
        bbbb: 3,
    };
    println!("{:?}", (b.b, b.bb, b.bbb, b.bbbb));
    b.print_contents();
}
