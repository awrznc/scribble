

pub trait PseudoDimensional {
    // fn roll( &mut self, dim_x: usize, dim_y: usize, shift_x: i32, shift_y: i32 );
    fn roll( &self, dim_x: usize, dim_y: usize, shift_x: i32, shift_y: i32 ) -> Self;

    fn print(&self, slice: usize);
    fn sub_square(
        &self,
        dim_x: usize,
        dim_y: usize,
        square_size_x: usize,
        square_size_y: usize,
        value: f32
    ) -> Self;
    fn frame(&self, x: usize, y: usize, num: f32) -> Self;
}


impl PseudoDimensional for Vec<f32> {

    /// 行列を回転させる
    ///
    /// # Arguments
    ///
    /// | name | desc |
    /// | --- | --- |
    /// | self    | 回転対象の行列 |
    /// | dim_x   | 回転対象の行列のxサイズ |
    /// | dim_y   | 回転対象の行列のyサイズ |
    /// | shift_x | 右にシフトする |
    /// | shift_y | 下にシフトする |
    fn roll(
        &self,
        dim_x: usize,
        dim_y: usize,
        shift_x: i32,
        shift_y: i32
    )-> Self {

        assert_eq!(dim_x * dim_y, self.len());

        let mut edit_vec: Self = self.clone();

        if shift_x > 0 {
            for i in 0..dim_x {
                edit_vec[((i)*dim_x)..((i+1)*dim_x)].rotate_right(shift_x as usize);
            }
        } else if shift_x < 0 {
            for i in 0..dim_x {
                edit_vec[((i)*dim_x)..((i+1)*dim_x)].rotate_left((-1 * shift_x) as usize);
            }
        }

        if shift_y > 0 {
            edit_vec.rotate_right((shift_y as usize) * dim_x);
        } else if shift_y < 0 {
            edit_vec.rotate_left(((-1 * shift_y) as usize) * dim_x);
        }

        edit_vec
    }
    // fn roll(
    //     &mut self,
    //     dim_x: usize,
    //     dim_y: usize,
    //     shift_x: i32,
    //     shift_y: i32
    // ) {
    //     assert_eq!(dim_x * dim_y, self.len());

    //     if shift_x > 0 {
    //         for i in 0..dim_x {
    //             self[((i)*dim_x)..((i+1)*dim_x)].rotate_right(shift_x as usize);
    //         }
    //     } else if shift_x < 0 {
    //         for i in 0..dim_x {
    //             self[((i)*dim_x)..((i+1)*dim_x)].rotate_left((-1 * shift_x) as usize);
    //         }
    //     }

    //     if shift_y > 0 {
    //         self.rotate_right((shift_y as usize) * dim_x);
    //     } else if shift_y < 0 {
    //         self.rotate_left(((-1 * shift_y) as usize) * dim_x);
    //     }
    // }

    fn print(&self, slice: usize) {
        for (i, v) in self.iter().enumerate() {
            print!("{:?} ", v);
            if (i+1) % slice == 0 {
                print!("\n");
            }
        }
    }

    fn sub_square(
        &self,
        dim_x: usize,
        dim_y: usize,
        square_size_x: usize,
        square_size_y: usize,
        value: f32
    ) -> Self {
        let mut edit_vec: Self = self.clone();
        let mut len: usize = edit_vec.len();
        let x_low = (dim_x/2) - (square_size_x/2);
        let x_high = (dim_x/2) + (square_size_x/2);
        let y_low = (dim_y/2) - (square_size_y/2);
        let y_high = (dim_y/2) + (square_size_y/2);

        for i in y_low..y_high {
            for v in edit_vec[ (x_low+(i*dim_x))..(x_high+(i*dim_x)) ].iter_mut() {
                *v -= value;
            }
        }

        edit_vec
    }

    fn frame(&self, x: usize, y: usize, num: f32) -> Self {
        let mut edit_vec: Self = self.clone();

        // 以下のような形でRangeを定義し、枠を描くようにして値を上書く
        // 0 0 0 3
        // 2 . . 3
        // 2 . . 3
        // 2 1 1 1
        vec![
            ( 0..(x-1) ).step_by(1),                // 0
            ( ((x*(y-1)+1) )..(x*y) ).step_by(1),   // 1
            ( x..(x*y) ).step_by(x),                // 2
            ( (x-1)..(x*(y-1)) ).step_by(x)         // 3
        ].into_iter().flatten().for_each( |v|
            edit_vec[v] = num
        );

        edit_vec
    }
}


// pub trait Math<T> {
//     // fn add(&mut self, other: T);
//     fn add(&self, other: T) -> Self;
//     fn sub(&self, other: T) -> Self;
//     fn mul(&self, other: T, size: usize) -> Self;
//     fn div(&self, other: T) -> Self;
// }


// impl Math<&Vec<f32>> for Vec<f32> {

//     /// 行列同士の足し算
//     fn add(&self, other: f32) -> Self {
//         self.iter().map(| v |
//             v + other
//         ).collect::< Vec<f32> >()
//     }
//     // fn add(&mut self, other: &Self) {
//     //     for i in (0..self.len()) {
//     //         self[i] = self[i] + other[i];
//     //     }
//     // }

//     /// 行列同士の引き算
//     fn sub(&self, other: &Self) -> Self {
//         self.iter().enumerate().map(|(i, v)|
//             v - other[i]
//         ).collect::< Vec<f32> >()
//     }

//     /// 行列同士の掛け算
//     fn mul(&self, other: &Self, size: usize) -> Self {
//         // let mut edit_vec = vec![0.0; self.len()];

//         // for i in 0..(size) {
//         //     for j in 0..(size) {
//         //         for k in 0..(size) {
//         //             edit_vec[i*size+j] += self[i*size+k] * other[k*size+j];
//         //         }
//         //     }
//         // }
//         // edit_vec
//         self.iter().enumerate().map(|(i, v)|
//             v * other[i]
//         ).collect::< Vec<f32> >()
//     }

//     /// 行列同士の割り算
//     fn div(&self, other: &Self) -> Self {
//         vec![0.0]
//     }
// }

pub trait Math<T> {
    fn add(&self, other: T) -> Self;
    fn sub(&self, other: T) -> Self;
    fn mul(&self, other: T, size: usize) -> Self;
    fn div(&self, other: T) -> Self;
}


impl Math<&Vec<f32>> for Vec<f32> {

    /// 行列同士の足し算
    fn add(&self, other: &Self) -> Self {
        self.iter().enumerate().map(|(i, v)|
            v + other[i]
        ).collect::< Vec<f32> >()
    }

    /// 行列同士の引き算
    fn sub(&self, other: &Self) -> Self {
        self.iter().enumerate().map(|(i, v)|
            v - other[i]
        ).collect::< Vec<f32> >()
    }

    /// 行列同士の掛け算
    fn mul(&self, other: &Self, size: usize) -> Self {
        // let mut edit_vec = vec![0.0; self.len()];

        // for i in 0..(size) {
        //     for j in 0..(size) {
        //         for k in 0..(size) {
        //             edit_vec[i*size+j] += self[i*size+k] * other[k*size+j];
        //         }
        //     }
        // }
        // edit_vec
        self.iter().enumerate().map(|(i, v)|
            v * other[i]
        ).collect::< Vec<f32> >()
    }

    /// 行列同士の割り算
    fn div(&self, other: &Self) -> Self {
        vec![0.0]
    }
}



impl Math<f32> for Vec<f32> {

    fn add(&self, other: f32) -> Self {
        self.iter().map(| v |
            v + other
        ).collect::< Vec<f32> >()
    }

    fn sub(&self, other: f32) -> Self {
        self.iter().map(| v |
            v - other
        ).collect::< Vec<f32> >()
    }

    fn mul(&self, other: f32, size: usize) -> Self {
        self.iter().map(| v |
            v * other
        ).collect::< Vec<f32> >()
    }

    fn div(&self, other: f32) -> Self {
        self.iter().map(| v |
            v / other
        ).collect::< Vec<f32> >()
    }
}

pub fn generate_random_vec(size: usize, low: f32, high: f32) -> Vec<f32> {
    use rand::{Rng, distributions::Uniform};

    rand::thread_rng()
        .sample_iter(Uniform::new(low, high))
        .take(size)
        .collect::<Vec<f32>>()
}

pub fn generate_meshgrid_vec(n: usize, low: f32, high: f32) -> Vec<f32> {
    let size = n * n;
    let scale = (high-low)/(size as f32);
    vec![low; size].iter().enumerate().map(|(i,v)|
        v + (i as f32 * scale)
    ).collect::< Vec<f32> >()
}
