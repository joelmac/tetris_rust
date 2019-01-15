extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::rect::Rect;
use std::time::Duration;
use std::thread::sleep;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
                           texture_creator: &'a TextureCreator<WindowContext>,
                           color: TextureColor,
                           width: u32,
                           height: u32) -> Option<Texture<'a>> {

    if let Ok(mut rect_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas.with_texture_canvas(&mut rect_texture, |texture| {
            match color {
                TextureColor::Green => texture.set_draw_color(Color::RGB(0,255,0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0,0,255)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(rect_texture)
    } else {
        None
    }
}

fn main() {
    const TEXTURE_SIZE: u32 = 32;

    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");

    let window = video_subsystem.window("Tetris", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut square_texture: Texture = texture_creator.create_texture_target(None, TEXTURE_SIZE, TEXTURE_SIZE)
        .expect("Failed to create a texture");
    canvas.with_texture_canvas(&mut square_texture, |texture| {
        texture.set_draw_color(Color::RGB(155,155,155));
        texture.clear();
    }).expect("Failed to color a texture");

    canvas.present();
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");
    let mut counter = 0;

    'running: loop  {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(155,155,155));
        canvas.clear();
        let rectangle = match counter > 60 {
            true => create_texture_rect(&mut canvas,&texture_creator,TextureColor::Green,TEXTURE_SIZE),
            false => create_texture_rect(&mut canvas,&texture_creator,TextureColor::Blue,TEXTURE_SIZE),
        }.unwrap();

        canvas.copy(&rectangle,
                    None,
                    Rect::new(0, 0, TEXTURE_SIZE, TEXTURE_SIZE))
            .expect("Couldn't copy texture into window");
        canvas.present();
        sleep(Duration::new(0,1_000_000_000u32 / 60));
        counter = (counter + 1) % 120;
    }
}
