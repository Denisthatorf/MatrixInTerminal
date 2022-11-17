extern crate termion;

use termion::color::Rgb;
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color, cursor};

use std::io::{stderr, stdout, Read, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

mod model;
use model::{Matrix};

pub const COLOR_MAP: [Rgb; 14] = [
    Rgb(0,0,0),
    Rgb(0,42,0),
    Rgb(0,42,0),
    Rgb(0,90,0),
    Rgb(0,125,0),
    Rgb(0,125,0),
    Rgb(0,170,0),
    Rgb(0,170,0),
    Rgb(0,170,0),
    Rgb(0,170,0),
    Rgb(0,170,0),
    Rgb(0,200,0),
    Rgb(0,200,0),
    Rgb(0,255,0),
];


fn draw<W: Write>(matrix: &Matrix, stdout: &mut W) {
        for x in 1..matrix.width {
            for y in 1..matrix.height {
                let cell = matrix.get_cell(x, y);
                write!(
                    stdout,
                    "{}{}{}{}",
                    cursor::Goto((x).try_into().unwrap(), (y).try_into().unwrap()),
                    color::Fg(COLOR_MAP[cell.intensity]),
                    color::Bg(COLOR_MAP[0]),
                    cell.value,
                )
                .unwrap();
            }
        }
}

fn run<W: Write, R: Read>(mut stdout: W, mut stdin: R, width: u16, height: u16) {
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    let before = Instant::now();
    let mut matrix = Matrix::new(width, height);

    loop {
        //read user input if input == q => quit
        let mut key_bytes = [0];
        stdin.read(&mut key_bytes).unwrap();

        if key_bytes[0] == b'q' {
            break;
        }

        //TODO: make use speed val
        const INTERVAL : u64 = 50u64;
        let now = Instant::now();
        let dt = (now.duration_since(before).subsec_millis() / INTERVAL as u32) as u64;

        if dt < INTERVAL {
            sleep(Duration::from_millis(INTERVAL - dt));
        }
        
        draw(&matrix, &mut stdout);
        matrix.update();
    }
}

fn main() {
    let stdout = stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();

    let stdin = async_stdin();

    let stderr = stderr();
    let _ = stderr.lock();

    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w, _)| w);
    let termheight = termsize.map(|(_, h)| h);

    run(
        stdout,
        stdin,
        termwidth.unwrap_or(70) + 1,
        termheight.unwrap_or(40) + 1,
    );
}