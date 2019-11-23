mod hoge;
use hoge::hoge::*;

fn main() {
    hello();
    fuga::hello();

    let hoge_object = Hoge::new(10);
    println!("{}", hoge_object.get_value());
}
