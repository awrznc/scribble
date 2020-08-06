// Windowsのコマンドプロンプトウィンドウを抑止
// - debug   ウィンドウを出す
// - release ウィンドウを抑止する
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use sdl2::rect::Point;

mod pseudo_dimensional;
use pseudo_dimensional::*;

// fn pseudo_dimensional() {

//     // 行列に利用する配列の長さ
//     let n = 100;

//     // 行列の行の幅
//     let x = n;

//     // 行列の列の幅
//     let y = n;

//     // 対称性を壊して行列を生成
//     let square_size = 10;
//     let mut u = generate_random_vec(x * y, 1.0, 1.1).sub_square(x, y, square_size, square_size, 0.5);
//     let mut v = generate_random_vec(x * y, 0.0, 0.1).sub_square(x, y, square_size, square_size, -0.25);

//     let dx = 0.01;
//     let dt = 1.;

//     let du = 1e-5;
//     let dv = 0.5e-5;
//     let (f, k) = (
//         generate_meshgrid_vec(n, 0.02, 0.06),
//         generate_meshgrid_vec(n, 0.05, 0.08)
//     );

//     // use std::thread;

//     for _x in 0..100 {

//         // let mut laplacian_u = u.clone();

//         // let handle = thread::spawn(move || {
//         //     let mut u1 = laplacian_u.clone();
//         //     let mut u2 = laplacian_u.clone();
//         //     let mut u3 = laplacian_u.clone();

//         //     laplacian_u.roll( x, y, 0, 1 );
//         //     u1.roll( x, y,  0, -1 );
//         //     u2.roll( x, y,  1,  0 );
//         //     u3.roll( x, y, -1,  0 );

//         //     for i in (0..laplacian_u.len()) {
//         //         laplacian_u[i] = laplacian_u[i] + u1[i] + u2[i] + u3[i];
//         //     }
//         // });

//         // let mut laplacian_v = v.clone();
//         // laplacian_v.roll( x, y, 0, 1 );

//         // let mut v1 = v.clone();
//         // let mut v2 = v.clone();
//         // let mut v3 = v.clone();
//         // v1.roll( x, y,  0, -1 );
//         // v3.roll( x, y, -1,  0 );
//         // v2.roll( x, y,  1,  0 );

//         // for i in (0..laplacian_v.len()) {
//         //     laplacian_v[i] = laplacian_v[i] + v1[i] + v2[i] + v3[i];
//         // }

//         // handle.join().unwrap();


//         // ラプラシアンの計算
//         let mut laplacian_u = u.clone().roll( x, y, 0, 1 )
//             .add(&u.roll( x, y,  0, -1 ))
//             .add(&u.roll( x, y,  1,  0 ))
//             .add(&u.roll( x, y, -1,  0 ))
//             .sub(&u.mul(4., n))
//             .div(dx*dx);

//         let mut laplacian_v = v.clone().roll( x, y, 0, 1 )
//             .add(&v.roll( x, y,  0, -1 ))
//             .add(&v.roll( x, y,  1,  0 ))
//             .add(&v.roll( x, y, -1,  0 ))
//             .sub(&v.mul(4., n))
//             .div(dx*dx);

//         // Gray-Scottモデル方程式
//         let dudt = &laplacian_u.mul( du, n )
//             .sub( &u.mul(&v, n).mul(&v, n) )
//             .add( &u.mul(-1.0, n).add(1.0).mul(&f, n) );

//         let dvdt = &laplacian_v.mul( dv, n )
//             .add( &u.mul(&v, n).mul(&v, n) )
//             .sub( &v.mul(&f.add(&k), n) );

//         u = u.add( &dudt.mul(dt, n) );
//         v = v.add( &dvdt.mul(dt, n) );
//     }
// }


// Screen dimension constants
const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    // initialize
    let sdl_context = sdl2::init().expect("SDL could not initialize!");
    let video_subsystem = sdl_context.video().expect("SDL could not initialize!");

    // window
    let window = video_subsystem
            .window("SDL Tutorial", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .expect("Window could not be created!");

    // canvas
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255,165,0));
    canvas.clear();
    canvas.present();

    // point
    // let point1 = Point::new(100, 100);
    // let point2 = Point::new(200, 200);
    // canvas.set_draw_color(Color::RGB(0,0,0));
    // canvas.draw_line(point1, point2).unwrap();

    // GSM
    let n = 100;
    let x = n;
    let y = n;
    let square_size = 10;
    let mut u = generate_random_vec(x * y, 1.0, 1.1).sub_square(x, y, square_size, square_size, 0.5);
    let mut v = generate_random_vec(x * y, 0.0, 0.1).sub_square(x, y, square_size, square_size, -0.25);
    let dx = 0.01;
    let dt = 1.;
    let du = 1e-5;
    let dv = 0.5e-5;
    let (f, k) = (
        generate_meshgrid_vec(n, 0.04, 0.04),
        generate_meshgrid_vec(n, 0.06, 0.06)
    );
    // let (f, k) = (
    //     generate_meshgrid_vec(n, 0.02, 0.06),
    //     generate_meshgrid_vec(n, 0.05, 0.08)
    // );

    // event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        for _ in (0..20) {
            // GSM
            let mut laplacian_u = u.clone().roll( x, y, 0, 1 )
                .add(&u.roll( x, y,  0, -1 ))
                .add(&u.roll( x, y,  1,  0 ))
                .add(&u.roll( x, y, -1,  0 ))
                .sub(&u.mul(4., n))
                .div(dx*dx);

            let mut laplacian_v = v.clone().roll( x, y, 0, 1 )
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

            for i in (0..(x*y)) {
                let point = Point::new(
                    (i % x + 200) as i32,
                    (i / y + 200) as i32
                );

                let mut input: u8 = 0;
                if u[i] > 1.0 {
                    input = 255;
                } else {
                    input = ( (255.0 / 100.0) * (u[i] * 100.0) ) as u8;
                }

                canvas.set_draw_color(Color::RGB(input, input, input));
                canvas.draw_point(point).unwrap();
            }
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
