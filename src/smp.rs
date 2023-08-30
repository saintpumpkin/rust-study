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
}

impl SmpController {
    pub fn new() -> SmpController {
        SmpController {
            x: 0,
            y: 0,
        }
    }

    pub fn move_up(&mut self) {
        self.y = std::cmp::max(0, self.y - 1);
    }

    pub fn move_down(&mut self) {
        self.y = self.y + 1;
    }

    pub fn move_right(&mut self) {
        self.x = self.x + 1;
    }

    pub fn move_left(&mut self) {
        self.x = std::cmp::max(0, self.x - 1);
    }

    pub fn update(&self, render: &mut SmpRender) {
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
        Smp {
            controller: SmpController::new(),
            input: SmpInput::<SmpController>::new(),
            render: SmpRender::new(),
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
            thread::sleep(Duration::from_millis(100));
        }
    }

    
}

pub fn run() {
    let mut smp = Smp::new();
    smp.init();
    smp.update();
}