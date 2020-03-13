extern crate image;

mod pseudo_dimensional;
use pseudo_dimensional::*;

fn main() {
    pseudo_dimensional();
}


fn pseudo_dimensional() {

    // 行列に利用する配列の長さ
    let n = 800;

    // 行列の行の幅
    let x = n;

    // 行列の列の幅
    let y = n;

    // 対称性を壊して行列を生成
    let square_size = 20;
    let mut u = generate_random_vec(x * y, 1.0, 1.1).sub_square(x, y, square_size, square_size, 0.5);
    let mut v = generate_random_vec(x * y, 0.0, 0.1).sub_square(x, y, square_size, square_size, -0.25);

    let dx = 0.01;
    let dt = 1.;

    let du = 2e-5;
    let dv = 1e-5;
    let (f, k) = (
        generate_meshgrid_vec(n, 0.02, 0.06),
        generate_meshgrid_vec(n, 0.05, 0.08)
    );

    for _x in 0..10000 {

        // ラプラシアンの計算
        let mut laplacian_u = u.roll( x, y, 0, 1 )
            .add(&u.roll( x, y,  0, -1 ))
            .add(&u.roll( x, y,  1,  0 ))
            .add(&u.roll( x, y, -1,  0 ))
            .sub(&u.mul(4., n))
            .div(dx*dx);

        let mut laplacian_v = v.roll( x, y, 0, 1 )
            .add(&v.roll( x, y,  0, -1 ))
            .add(&v.roll( x, y,  1,  0 ))
            .add(&v.roll( x, y, -1,  0 ))
            .sub(&v.mul(4., n))
            .div(dx*dx);

        // Gray-Scottモデル方程式
        let dudt = &laplacian_u.mul( du, n )
            .sub( &u.mul(&v, n).mul(&v, n) )
            .add( &u.mul(-1.0, n).add(1.0).mul(&f, n) );

        let dvdt = &laplacian_v.mul( dv, n )
            .add( &u.mul(&v, n).mul(&v, n) )
            .sub( &v.mul(&f.add(&k), n) );

        u = u.add( &dudt.mul(dt, n) );
        v = v.add( &dvdt.mul(dt, n) );
    }

    // 書き出し
    let mut imgbuf = image::ImageBuffer::new(x as u32, y as u32);

    for (imgx, imgy, pixel) in imgbuf.enumerate_pixels_mut() {

        let color = u[(imgx + (x as u32 * imgy)) as usize];
        let mut input: u8 = 0;

        if color < 0.0 {
            input = 0;
        } else if color > 1.0 {
            input = 255;
        } else {
            input = ( (255.0 / 100.0) * (color * 100.0) ) as u8;
        }

        *pixel = image::Rgb(
            [
                input,
                input,
                input
            ]
        );
    }

    imgbuf.save("output/fractal.png").unwrap();
}
