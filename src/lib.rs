
pub mod tetrimino;
pub mod tetris;
pub mod sdl_events;

#[cfg(test)]
mod tests {
    use crate::tetrimino::{Tetrimino,TetriminoI,TetriminoGenerator};

    #[test]
    fn make_tetrimino() {
       assert_eq!(TetriminoI::new().y,0);
    }
}
