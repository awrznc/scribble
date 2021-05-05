use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn main() {
    let mut vec = vec![0; 200000];

    {
        let file = File::open("src/log.csv").unwrap();
        let mut contents = String::new();
        BufReader::new(file).read_to_string(&mut contents).unwrap();
        for line in contents.lines(){
            let tuple = line.split(",").map(|v|v.parse().unwrap()).collect::<Vec<usize>>();
            vec[tuple[0]] = tuple[1];
        }
    }

    println!("{:?}", vec[2000]);
}
