// Windowsのコマンドプロンプトウィンドウを抑止
// - debug   ウィンドウを出す
// - release ウィンドウを抑止する
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    // initialize
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // window
    let window = video_subsystem
            .window("SDL", 640, 480)
            .position_centered()
            .build()
            .unwrap();

    // canvas
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.clear();
    canvas.present();

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
