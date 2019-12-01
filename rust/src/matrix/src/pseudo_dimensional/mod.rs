use std::{thread, time};


pub fn pseudo_dimensional() {
    // 行列に利用する配列の長さ
    let n = 4;

    // 行列の行の幅
    let x = n;

    // 行列の列の幅
    let y = n;

    // 対称性を壊して行列を生成
    let mut u = generate_random_vec(x * y, 1.0, 1.1).sub_square(x, y, 2, 2, 0.5);
    let mut v = generate_random_vec(x * y, 1.0, 1.1).sub_square(x, y, 2, 2, 0.75);

    let dx = 0.01;
    let dt = 1.;

    let du = 2e-5;
    let dv = 1e-5;
    let (f, k) = (0.04, 0.06);  // amorphous

    u.print(n);

    loop {
        let mut laplacian_u = u.roll( x, y, 0, 1 )
            .add(&u.roll( x, y,  0, -1 ))
            .add(&u.roll( x, y,  1,  0 ))
            .add(&u.roll( x, y, -1,  0 ))
            .sub(&u.mul(4.))
            .div(dx*dx);

        let mut laplacian_v = u.roll( x, y, 0, 1 )
            .add(&u.roll( x, y,  0, -1 ))
            .add(&u.roll( x, y,  1,  0 ))
            .add(&u.roll( x, y, -1,  0 ))
            .sub(&u.mul(4.))
            .div(dx*dx);

        u = u.add( &laplacian_u.mul(dt).mul(du) );
        v = v.add( &laplacian_v.mul(dt).mul(dv) );

        u.print(x);
        
        thread::sleep( time::Duration::from_millis(1000) );
    }
}


trait PseudoDimensional {
    fn roll( &self, dim_x: usize, dim_y: usize, shift_x: i32, shift_y: i32 ) -> Self;
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn print(&self, slice: usize);
    fn sub_square(
        &self,
        dim_x: usize,
        dim_y: usize,
        square_size_x: usize,
        square_size_y: usize,
        value: f32
    ) -> Self;
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

    fn add(&self, other: &Self) -> Self {
        self.iter().enumerate().map(|(i, v)|
            v + other[i]
        ).collect::< Vec<f32> >()
    }

    fn sub(&self, other: &Self) -> Self {
        self.iter().enumerate().map(|(i, v)|
            v - other[i]
        ).collect::< Vec<f32> >()
    }

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

        println!("edit_vec: {:?}", edit_vec);
        edit_vec
    }
}

trait Math<T> {
    fn mul(&self, other: T) -> Self;
    fn div(&self, other: T) -> Self;
}

impl Math<&Vec<f32>> for Vec<f32> {
    /// 行列同士をかけ合わせる
    fn mul(&self, other: &Self) -> Self {
        const HW: usize = 4;
        let mut edit_vec = vec![0.0; self.len()];

        for i in 0..(HW) {
            for j in 0..(HW) {
                for k in 0..(HW) {
                    edit_vec[i*HW+j] += self[i*HW+k] * other[k*HW+j];
                }
            }
        }
        edit_vec
    }

    fn div(&self, other: &Self) -> Self {
        vec![0.0]
    }
}

impl Math<f32> for Vec<f32> {
    /// 行列同士をかけ合わせる
    fn mul(&self, other: f32) -> Self {
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


fn generate_random_vec(size: usize, low: f32, high: f32) -> Vec<f32> {
    use rand::{Rng, distributions::Uniform};

    rand::thread_rng()
        .sample_iter(Uniform::new(low, high))
        .take(size)
        .collect::<Vec<f32>>()
}