use std::io;
use std::thread;
use std::time::Duration;

use crate::smp_input::SmpInput;
use crate::smp_render::SmpRender;
use crossterm::{
    event::{KeyCode},
};

pub struct SmpController {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    add_value: i32,
}

impl SmpController {
    pub fn new(width: usize, height: usize) -> SmpController {
        SmpController {
            x: 0,
            y: 0,
            width: width,
            height: height,
            add_value: 2,
        }
    }

    pub fn move_up(&mut self) {
        self.y = match self.y.checked_sub(1) {
            Some(y) => y,
            None => 0,
        };
    }

    pub fn move_down(&mut self) {
        self.y = (self.y + 1).clamp(0, self.height);
        if (self.height < self.y) {
            println!("overflow!!!!!");
        }
    }

    pub fn move_right(&mut self) {
        self.x = (self.x + 2).clamp(0, self.width);
        if (self.width < self.x) {
            println!("overflow!!!!!");
        }
    }

    pub fn move_left(&mut self) {
        self.x = match self.x.checked_sub(2) {
            Some(x) => x,
            None => 0,
        };
    }

    fn auto_movement(&mut self) {
        let mut cur_x: i32 = self.x as i32;
        if cur_x == 70 {
            self.add_value = -2;
        } else if cur_x == 0{
            self.add_value = 2;
        }
        cur_x = cur_x + self.add_value;
        self.x = cur_x as usize;
    }

    pub fn update(&mut self, render: &mut SmpRender) {
        self.auto_movement();
        render.set_object_position(self.x, self.y)
    }
}

pub struct Smp {
    controller: SmpController,
    input: SmpInput<SmpController>,
    render: SmpRender,
}

impl Smp {
    fn new() -> Smp {
        let render = SmpRender::new();
        Smp {
            controller: SmpController::new(render.get_width(), render.get_height()),
            input: SmpInput::<SmpController>::new(),
            render: render,
        }
    }

    fn init(&mut self) {
        self.input.add_key(&KeyCode::Enter, Box::new(|arg| {println!("Enter!!");}));
        self.input.add_key(&KeyCode::Up, Box::new(|arg| {arg.move_up(); }));
        self.input.add_key(&KeyCode::Down, Box::new(|arg| {arg.move_down(); }));
        self.input.add_key(&KeyCode::Right, Box::new(|arg| {arg.move_right(); }));
        self.input.add_key(&KeyCode::Left, Box::new(|arg| {arg.move_left(); }));
    }

    fn update(&mut self) {
        loop {
            self.input.update(&mut self.controller);
            self.controller.update(&mut self.render);
            self.render.render();
            thread::sleep(Duration::from_millis(10));
        }
    }

    
}

pub fn run() {
    let mut smp = Smp::new();
    smp.init();
    smp.update();
}