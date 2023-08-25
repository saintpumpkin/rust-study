#![allow(unused_imports, unused_variables, dead_code)]

use std::thread;
use std::time::Duration;
use console::Term;

// https://github.com/console-rs/console/tree/master/examples

pub fn run() {
    println!("smp_render run");

    let term = Term::stdout();
    term.write_line("Hello World!");
    thread::sleep(Duration::from_millis(2000));
    //term.clear_line();
    term.clear_last_lines(1);
    thread::sleep(Duration::from_millis(2000));
}