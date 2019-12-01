pub fn srt() {
    let a = MultiDimensional::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]);
    let b = MultiDimensional::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]);
    let c = a + b;
    println!("{:?}", c);
    c.range(0..16);
}

#[derive(Debug)]
struct MultiDimensional {
    dimensional: Vec<usize>,
    vector: Vec<f32>
}

impl MultiDimensional {

    fn new(ds: Vec<usize>, vec: Vec<f32>) -> Self {
        Self { dimensional: ds, vector: vec }
    }

    fn range(&self, range: std::ops::Range<i32> ) -> Self {
        println!("{:?}", range);

        Self { dimensional: vec![2, 2], vector: vec![1.0, 2.0, 3.0, 4.0] }
    }
}

use std::ops::Add;

impl Add for MultiDimensional {
    type Output = Self;

    fn add(self, vec: MultiDimensional) -> Self {
        Self {
            dimensional: self.dimensional,
            vector: add_logic(&self.vector, &vec.vector)
        }
    }
}

fn add_logic(a: &Vec<f32>, b: &Vec<f32>) -> Vec<f32> {
    a.iter().enumerate().map(|(i, y)|
        y + b[i]
    ).collect::< Vec<f32> >()
}