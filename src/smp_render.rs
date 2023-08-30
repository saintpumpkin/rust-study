#![allow(unused_imports, unused_variables, dead_code)]

use std::io;
use std::thread;
use std::time::Duration;

use console::{style, Term};

// https://github.com/console-rs/console/tree/master/examples

fn write_chars() -> io::Result<()> {
    let term = Term::stdout();
    let (heigth, width) = term.size();
    for x in 0..width {
        for y in 0..heigth {
            term.move_cursor_to(x as usize, y as usize)?;
            let text = if (x + y) % 2 == 0 {
                format!("{}", style(x % 10).black().on_red())
            } else {
                format!("{}", style(x % 10).red().on_black())
            };

            term.write_str(&text)?;
            thread::sleep(Duration::from_micros(600));
        }
    }
    Ok(())
}

pub struct SmpRender {
    x: usize,
    y: usize,
    width: usize,
    heigth: usize,
    //buffer: Vec<char>
}

impl SmpRender {
    pub fn new() -> SmpRender {
        let term = Term::stdout();
        let (heigth, width) = term.size();
        SmpRender {
            x: 0,
            y: 0,
            width: width as usize,
            heigth: heigth as usize,
            //buffer: Vec::<char>::new(),
        }
    }

    fn draw_background(&mut self) {
        //self.buffer.resize(self.width * self.heigth, '■');
        let term = Term::stdout();
        for x in 0..self.width {
            for y in 0..self.heigth {
                term.move_cursor_to(x as usize, y as usize);
                let text = format!("{}", style('■').black().on_red());
                term.write_str(&text);
            }
        }
    }

    fn draw_object(&mut self, x: usize, y: usize) {
        let term = Term::stdout();
        term.move_cursor_to(x, y);
        let text = format!("{}", style('○').black().on_green());
        term.write_str(&text);
    }

    pub fn set_object_position(&mut self, x:usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&mut self) {
        //let term = Term::stdout();
        //self.draw_background();
        self.draw_object(self.x, self.y);
        //let string = self.buffer.iter().collect::<String>();
        //term.write_str(&string);
    }
}

pub fn run() {
    println!("smp_render run");

    // let term = Term::stdout();
    // term.write_line("Hello World!");
    // thread::sleep(Duration::from_millis(2000));
    // //term.clear_line();
    // term.clear_last_lines(1);
    // thread::sleep(Duration::from_millis(2000));

    //write_chars().unwrap();

    let mut render = SmpRender::new();
    render.render();
    thread::sleep(Duration::from_millis(5000));
}