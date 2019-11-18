fn main() {

    // import numpy as np
    use crate::TDVec;

    // u = np.arange(16).reshape(4,4)
    let origin: Vec<Vec<i32>> = vec![
        vec![  0,  1,  2,  3 ],
        vec![  4,  5,  6,  7 ],
        vec![  8,  9, 10, 11 ],
        vec![ 12, 13, 14, 15 ],
    ];

    // u * 4
    let u = vec![
        vec![  0,  4,  8, 12 ],
        vec![ 16, 20, 24, 28 ],
        vec![ 32, 36, 40, 44 ],
        vec![ 48, 52, 56, 60 ],
    ];

    // print(np.roll(u, 1, 0) + np.roll(u, -1, 0) + np.roll(u, 1, 1) + np.roll(u, -1, 1) - 4 * u)
    let got = origin.roll( 0,  1)
        .add(&origin.roll( 0, -1))
        .add(&origin.roll( 1,  0))
        .add(&origin.roll(-1,  0))
        .subtract(&u);

    println!("{:?}", got);
    // [[ 20  16  16  12]
    //  [  4   0   0  -4]
    //  [  4   0   0  -4]
    //  [-12 -16 -16 -20]]
}


trait TDVec {
    fn roll(&self, x_shift_num: i32, y_shift_num: i32) -> Self;
    fn add(&self, vec: &Self) -> Self;
    fn subtract(&self, vec: &Self) -> Self;
}

impl TDVec for Vec<Vec<i32>> {
    fn roll(&self, x_shift_num: i32, y_shift_num: i32)-> Self {

        let mut edit_vec: Self = self.clone();

        if x_shift_num > 0 {
            for x in 0..edit_vec.len() {
                edit_vec[x].rotate_right(x_shift_num as usize);
            }
        } else if x_shift_num < 0 {
            for x in 0..edit_vec.len() {
                edit_vec[x].rotate_left((-1 * x_shift_num) as usize);
            }
        }

        if y_shift_num > 0 {
            edit_vec.rotate_right(y_shift_num as usize);
        } else if y_shift_num < 0 {
            edit_vec.rotate_left((-1 * y_shift_num) as usize);
        }

        edit_vec
    }

    fn add(&self, vec: &Self) -> Self {
        self.iter().enumerate().map(|(i, y)|
            y.iter().enumerate().map(|(j, x)|
                (x + vec[i][j]) as i32
            ).collect::< Vec<i32> >()
        ).collect::< Vec<Vec<i32>> >()
    }

    fn subtract(&self, vec: &Self) -> Self {
        self.iter().enumerate().map(|(i, y)|
            y.iter().enumerate().map(|(j, x)|
                (x - vec[i][j]) as i32
            ).collect::< Vec<i32> >()
        ).collect::< Vec<Vec<i32>> >()
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn roll() {
        let origin: Vec<Vec<i32>> = vec![
            vec![  0,  1,  2,  3 ],
            vec![  4,  5,  6,  7 ],
            vec![  8,  9, 10, 11 ],
            vec![ 12, 13, 14, 15 ]
        ];

        let expect: Vec<Vec<i32>> = vec![
            vec![ 15, 12, 13, 14 ],
            vec![  3,  0,  1,  2 ],
            vec![  7,  4,  5,  6 ],
            vec![ 11,  8,  9, 10 ],
        ];

        use crate::TDVec;
        let got: Vec<Vec<i32>> = origin.roll(1, 1);
        assert_eq!(4, got.len());

        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(got[i][j], expect[i][j]);
            }
        }
    }

    #[test]
    fn add() {
        let origin: Vec<Vec<i32>> = vec![
            vec![  0,  1,  2,  3 ],
            vec![  4,  5,  6,  7 ],
            vec![  8,  9, 10, 11 ],
            vec![ 12, 13, 14, 15 ],
        ];

        let expect: Vec<Vec<i32>> = vec![
            vec![  0,  2,  4,  6 ],
            vec![  8, 10, 12, 14 ],
            vec![ 16, 18, 20, 22 ],
            vec![ 24, 26, 28, 30 ]
        ];

        use crate::TDVec;
        let got = origin.add(&origin);

        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(got[i][j], expect[i][j]);
            }
        }
    }

    #[test]
    fn merge_test() {
        let origin: Vec<Vec<i32>> = vec![
            vec![  0,  1,  2,  3 ],
            vec![  4,  5,  6,  7 ],
            vec![  8,  9, 10, 11 ],
            vec![ 12, 13, 14, 15 ],
        ];

        let expect = vec![
            vec![  20,  16,  16,  12 ],
            vec![   4,   0,   0,  -4 ],
            vec![   4,   0,   0,  -4 ],
            vec![ -12, -16, -16, -20 ],
        ];

        let u = vec![
            vec![  0,  4,  8, 12 ],
            vec![ 16, 20, 24, 28 ],
            vec![ 32, 36, 40, 44 ],
            vec![ 48, 52, 56, 60 ],
        ];


        use crate::TDVec;
        let got = origin.roll( 0,  1)
            .add(&origin.roll( 0, -1))
            .add(&origin.roll( 1,  0))
            .add(&origin.roll(-1,  0))
            .subtract(&u);

        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(got[i][j], expect[i][j]);
            }
        }
    }
}