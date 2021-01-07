// Windowsのコマンドプロンプトウィンドウを抑止
// - debug   ウィンドウを出す
// - release ウィンドウを抑止する
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

use sdl2::rect::Point;

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

    // draw line
    let point1 = Point::new(100, 100);
    let point2 = Point::new(200, 200);
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.draw_line(point1, point2).unwrap();

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

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
