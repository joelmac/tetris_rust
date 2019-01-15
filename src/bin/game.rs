extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, Texture, TextureCreator};
//use sdl2::image::{LoadTexture, INIT_PNG, INIT_JPG};

use std::thread::sleep;
use std::time::{Duration, SystemTime};
use tetris_emulator::tetris::Tetris;
use tetris_emulator::sdl_events::handle_events;

const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
const TETRIS_HEIGHT: u32 = 40;

#[derive(Clone, Copy)]
enum TextureColor {
    Green,
    Blue,
}

fn print_game_information(tetris: &Tetris) {
    println!("Game over...");
    println!("Score:        {}", tetris.score);
    println!("Current level:    {}", tetris.current_level);
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>,
                           texture_creator: &'a TextureCreator<WindowContext>,
                           color: TextureColor,
                           width: u32,
                           height: u32) -> Option<Texture<'a>> {

    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            match color {
                TextureColor::Green => texture.set_draw_color(Color::RGB(0,255,0)),
                TextureColor::Blue => texture.set_draw_color(Color::RGB(0,0,255)),
            }
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn is_time_over(tetris: &Tetris, timer: &SystemTime) -> bool{
    match timer.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_secs() as u32 * 1000 + elapsed.subsec_nanos() / 1_000_000;
            millis > LEVEL_TIMES[tetris.current_level as usize - 1]
        }
        Err(_) => false,
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem.");
    let width = 600;
    let height = 800;

    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();

    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump!");

    let grid_x = (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2;

    let window = video_subsystem.window("Tetris", width, height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let grid = create_texture_rect(&mut canvas,
                                   &texture_creator,
                                   Color::RGB(0, 0, 0),
                                   TETRIS_HEIGHT as u32 * 10,
                                   TETRIS_HEIGHT as u32 * 16).expect("Failed to create a texture");

    let border = create_texture_rect(&mut canvas,
                                     &texture_creator,
                                     Color::RGB(255, 255, 255),
                                     TETRIS_HEIGHT as u32 * 10 + 20,
                                     TETRIS_HEIGHT as u32 * 16 + 20).expect("Failed to create a texture");

    loop {
        if match timer.elapsed() {
            Ok(elapsed) => elapsed.as_secs() >= 1,
            Err(_) => false,
        } {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        if tetris.current_piece.is_none() {
            let current_piece = tetris_emulator::tetrimino::create_new_tetrimino();
            if !current_piece.test_current_position(&tetris.game_map) {
                print_game_information(&tetris);
                break
            }
            tetris.current_piece = Some(current_piece);
        }
        let mut quit = false;
        if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {
                //We need to draw our current tetrimino in here.
            }
        }
        if quit {
            print_game_information(&tetris);
            break
        }

        //We need to draw the game map in here.
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
