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
    // let (f, k) = (0.04, 0.06);  // amorphous
    let (f, k) = (
        generate_meshgrid_vec(n, 0.02, 0.06),
        generate_meshgrid_vec(n, 0.05, 0.08)
    );

    // loop {
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

        // let u_frame: Vec<f32> = u.frame( x, y, 1.0 );
        // let v_frame: Vec<f32> = v.frame( x, y, 1.0 );

        // // ラプラシアンの計算
        // let mut laplacian_u = u_frame.roll( x, y, 0, 1 )
        //     .add(&u_frame.roll( x, y,  0, -1 ))
        //     .add(&u_frame.roll( x, y,  1,  0 ))
        //     .add(&u_frame.roll( x, y, -1,  0 ))
        //     .sub(&u_frame.mul(4., n))
        //     .div(dx*dx);

        // let mut laplacian_v = v_frame.roll( x, y, 0, 1 )
        //     .add(&v_frame.roll( x, y,  0, -1 ))
        //     .add(&v_frame.roll( x, y,  1,  0 ))
        //     .add(&v_frame.roll( x, y, -1,  0 ))
        //     .sub(&v_frame.mul(4., n))
        //     .div(dx*dx);

        // Gray-Scottモデル方程式
        let dudt = &laplacian_u.mul( du, n )
            .sub( &u.mul(&v, n).mul(&v, n) )
            .add( &u.mul(-1.0, n).add(1.0).mul(&f, n) );

        // let dvdt = &laplacian_v.mul( dv, n )
        //     .add( &u.mul(&v, n).mul(&v, n) )
        //     .sub( &v.mul(f + k, n) );

        let dvdt = &laplacian_v.mul( dv, n )
            .add( &u.mul(&v, n).mul(&v, n) )
            .sub( &v.mul(&f.add(&k), n) );

        // 1.0291159 1.0932871 1.034722 1.0864173
        // 1.0041091 0.5053942 0.5804914 1.0983924
        // 1.0274676 0.55572546 0.56670177 1.0330337
        // 1.0115367 1.0324266 1.0597049 1.0166609

        // 0.0027701973 0.032793533 0.021081006 0.09417218
        // 0.0851377 0.2798852 0.34835383 0.006232727
        // 0.025903989 0.30050436 0.31463027 0.020051003
        // 0.05094538 0.0006664276 0.035443913 0.008024585

        // 0.17808217 0.65140575 0.7666073 0.13319351
        // 0.116804734 0.34955287 0.41879517 0.11816271
        // 0.117467605 0.3602184 0.4301654 0.119875155
        // 0.16994534 0.6412563 0.75042367 0.13109988

        u = u.add( &dudt.mul(dt, n) );
        v = v.add( &dvdt.mul(dt, n) );
    }

    // u.print(n);
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

        // println!("{}, {}", input, color);
        // let input = ( ( color - 0.0 ) / ( 1.0 - 0.0 ) * 255.0 ) as u8;
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
