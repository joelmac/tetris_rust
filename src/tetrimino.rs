extern crate rand;
type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

#[derive(Debug)]
pub struct Tetrimino {
    pub states: States,
    pub x: isize,
    pub y: usize,
    pub current_state: u8,
}

impl Tetrimino {

    fn test_position(&self, game_map: &[Vec<u8>],
                     tmp_state: usize, x: isize, y: usize) -> bool {
        for decal_y in 0..4 {
            for decal_x in 0..4 {
                let x = x + decal_x;
                if self.states[tmp_state][decal_y][decal_x as usize] != 0 
                    && (y + decal_y >= game_map.len() 
                        || x < 0 
                        || x as usize >= game_map[y + decal_y].len() 
                        || game_map[y + decal_y][x as usize] != 0) {
                    return false;
                }
            }
        }
        return true;
    }


    pub fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }


    pub fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool {
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) == true {
            self.x = new_x as isize;
            self.y = new_y;
            true
        } else {
            false
        }
    }
}

pub trait TetriminoGenerator {
    fn new() -> Tetrimino;
}


pub struct TetriminoI;

impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![1, 1, 1, 1],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]],
                         vec![vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0],
                              vec![0, 1, 0, 0]]],
            x: 4,
            y: 0,
            current_state: 0,
        }
    }
}


pub fn create_new_tetrimino() -> Tetrimino {
    let rand_nb = rand::random::<u8>() % 1;
    match rand_nb {
        0 => TetriminoI::new(),
        _ => unreachable!(),
    }
}


