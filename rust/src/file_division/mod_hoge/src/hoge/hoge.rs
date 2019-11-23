
pub struct Hoge {
    value: u8
}

impl Hoge {

    pub fn new(n: u8)-> Hoge {
        Hoge { value: n }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}

pub fn hello() {
    println!("Hello, world!");
}

pub mod fuga {
    pub fn hello() {
        println!("Hello, world!");
    }
}

