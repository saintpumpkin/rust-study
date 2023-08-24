#![allow(unused_imports, unused_variables, dead_code)]

use std::{io, io::SeekFrom, time::Duration};
use crossterm::{
    cursor::position,
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::collections::HashMap;

pub struct SmpInput<T> {
    key_map: HashMap<KeyCode, Box<dyn Fn(&mut T)>>,
}

// impl std::cmp::PartialEq for SmpInput {
//     fn eq(&self, other: &Self) -> bool {
//         let eq = true;
//         for (key, callback) in &self.key_map {
//             other.get(key)
//         }
//         eq
//     }
// }

impl<T> SmpInput<T> {
    fn new() -> SmpInput<T> {
        SmpInput {
            key_map: HashMap::new(),
        }
    }

    fn add_key(&mut self, key: &KeyCode, callback: Box<dyn Fn(&mut T)>) {
        self.key_map.insert(key.clone(), callback);
    }

    fn get(&self, key: &KeyCode) -> Option<&Box<dyn Fn(&mut T)>> {
        self.key_map.get(key)
    }

    fn on_input_key(&self, key: &KeyCode, arg: &mut T) {
        match self.get(key) {
            Some(callback) => callback(arg),
            None => println!("key callback not exists"),
        };
    }

    fn update(&self, arg: &mut T) -> io::Result<()> {
        if poll(Duration::from_millis(1_00))? {
            if let Event::Key(KeyEvent { code, modifiers, kind, state }) = read()? {
                match (kind) {
                    (KeyEventKind::Press) => {
                        self.on_input_key(&code, arg);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

#[test]
fn test_smp_input_new() {
    assert!(SmpInput::<i32>::new().key_map.is_empty());
}

#[test]
fn test_smp_input_callback() {
    let mut value = 0;
    let mut input = SmpInput::new();
    input.add_key(&KeyCode::Enter, Box::new(|arg| {*arg = 1; println!("Enter!!");}));
    input.on_input_key(&KeyCode::Enter, &mut value);
    assert_eq!(value, 1);
}

pub fn run() {
    println!("smp_input run");

    let mut value = 0;
    let mut last_value = value;
    let mut input = SmpInput::<i32>::new();
    input.add_key(&KeyCode::Enter, Box::new(|arg| {*arg = 0; println!("Enter!!");}));
    input.add_key(&KeyCode::Up, Box::new(|arg| {*arg = 1; println!("Up!!");}));
    input.add_key(&KeyCode::Down, Box::new(|arg| {*arg = 2; println!("Down!!");}));
    input.add_key(&KeyCode::Right, Box::new(|arg| {*arg = 3; println!("Right!!");}));
    input.add_key(&KeyCode::Left, Box::new(|arg| {*arg = 4; println!("Left!!");}));
    loop {
        match input.update(&mut value) {
            Ok(_) => {
                if value != last_value {
                    println!("current value: {value}");
                    last_value = value;
                }
            },
            Err(_) => {}
        }
    }
    
}