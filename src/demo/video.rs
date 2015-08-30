extern crate sdl2;

use sdl2::*;
use sdl2_ttf;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowPos};
use sdl2::render::{DriverIterator, Renderer};
use std::path::Path;

static SCREEN_WIDTH : i32 = 800;
static SCREEN_HEIGHT : i32 = 600;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as u32, $y as u32, $w as u32, $h as u32)
    )
);

pub fn main(filename: &Path) {
    let mut sdl_context = sdl2::init().unwrap();
    sdl2_ttf::init();

    let window = match Window::new(&sdl_context, "rust-sdl2 demo: Video", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600) {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window")
    };

    let mut renderer = match Renderer::from_window(window, DriverIterator::Auto) {
        Ok(renderer) => renderer,
        Err(err) => panic!("Failed to create renderer")
    };

    // Load a font
    let font = trying!(sdl2_ttf::Font::from_file(filename, 128));

    // render a surface, and convert it to a texture bound to the renderer
    let surface = trying!(font.render_str_blended("Hello Rust!", sdl2::pixels::Color::RGBA(255, 0, 0, 255)));
    let mut texture = trying!(renderer.create_texture_from_surface(&surface));

    let mut drawer = renderer.drawer();
    drawer.set_draw_color(sdl2::pixels::Color::RGBA(195, 217, 255, 255));
    drawer.clear();

    let (w, h) = { let q = texture.query(); (q.width, q.height) };

    drawer.copy(&mut texture, None, Some(rect!((SCREEN_WIDTH - w)/ 2, (SCREEN_HEIGHT - h)/ 2, w, h)));

    drawer.present();

    'mainloop: loop {
        for event in sdl_context.event_pump().poll_iter() {
            match event {
                Event::Quit{..} => break 'mainloop,
                Event::KeyDown {keycode: Keycode::Escape, ..} => break 'mainloop,
                _ => {}
            }
        }
    }

    sdl2_ttf::quit();
}
