use rand::Rng;

macro_rules! probability {
    ($prob:expr) => {
        rand::thread_rng().gen_range(0f32..1f32) < $prob
    };
}

const PROB_DRIP_SPAWN: f32 = 0.95;
const MIN_INTENSITY: usize = 4;
const MAX_INTENSITY: usize = 13;

const PROB_CHANCE: f32 = 0.95;
const PROB_DIM: f32 = 0.55;

#[derive(Copy, Clone)]
pub struct Drip {
    pub x: u16,
    pub y: u16,
    pub live: bool,
    pub bright: bool,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub value: char,
    //TODO: think about type
    pub intensity: usize,
}

pub struct Matrix {
    pub cells: Box<[Cell]>,
    pub drips: Box<[Drip]>,

    pub width: u16,
    pub height: u16,
}

impl Matrix {
    pub fn new(width: u16, height: u16) -> Matrix {
        let matrix: Matrix = Matrix {
            cells: vec![
                Cell {
                    value: '0',
                    intensity: 3
                };
                width as usize * height as usize
            ]
            .into_boxed_slice(),
            drips: vec![
                Drip {
                    x: 0,
                    y: 0,
                    live: false,
                    bright: false,
                };
                width as usize
            ]
            .into_boxed_slice(),
            width: width,
            height: height,
        };
        return matrix;
    }

    pub fn get_cell(&self, x: u16, y: u16) -> &Cell {
        return &self.cells[y as usize * self.width as usize + x as usize];
    }

    fn get_mut_cell(&mut self, x: u16, y: u16) -> &mut Cell {
        return &mut self.cells[y as usize * self.width as usize + x as usize];
    }

    fn set_cell_intensity(&mut self, x: u16, y: u16, value: usize) {
        self.cells[y as usize * self.width as usize + x as usize].intensity = value;
    }

    fn fade_n_change_matrix(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut cell: &mut Cell = self.get_mut_cell(x, y);
                if probability!(PROB_CHANCE) || cell.value == (0 as char) {
                    //TODO: get random char
                    cell.value = (33u8 + (rand::thread_rng().gen_range(0u8..80u8))) as char;
                }
                if probability!(PROB_DIM) {
                    if cell.intensity > 0 {
                        cell.intensity -= 1;
                    }
                }
            }
        }
    }

    fn try_add_drips(&mut self) {
        for i in 0..self.width {
            let drip: &mut Drip = &mut self.drips[i as usize];
            if drip.live == false {
                drip.live = true;
                drip.x = rand::thread_rng().gen_range(0..self.width);
                drip.y = 0;
                drip.bright = true; //TODO:
                return;
            }
        }
    }

    fn update_drips(&mut self) {
        for i in 0..self.width {
            let mut drip: Drip = self.drips[i as usize];
            if drip.live {
                if drip.bright {
                    self.set_cell_intensity(drip.x, drip.y, MAX_INTENSITY);
                } else {
                    self.set_cell_intensity(drip.x, drip.y, MIN_INTENSITY);
                }

                //rips die when they leave the screen
                drip.y += 1;
                if drip.y >= self.height {
                    drip.live = false;
                }

                self.drips[i as usize] = drip;
            }
        }
    }

    pub fn update(&mut self) {
        if probability!(PROB_DRIP_SPAWN) {
            self.try_add_drips();
        }
        self.update_drips();

        self.fade_n_change_matrix();
    }
}
