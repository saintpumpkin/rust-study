#![allow(unused_imports, unused_variables, dead_code)]

// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }

#[derive(Debug, Copy, Clone)]
struct Point<T> (T, T);

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.0  // + 10
    }

    // fn set_x(&mut self, x: T)
}

fn generic_data_type() {
    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    //println!("{integer:?} and {float:?}");
}

fn generic_method() {
    let p = Point(5, 10);
    println!("p.x = {}", p.x());
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn monomorphization() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The cat") // No name, cats won't respond to it anyway.
    }
}

fn greet<P: Pet>(pet: &P) {
    println!("Who's a cutie? {} is!", pet.name());
}

fn fn_trait() {
    let fido = Dog { name: "Fido".into() };
    greet(&fido);

    let captain_floof = Cat;
    greet(&captain_floof);
}

fn fn_trait_object() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat),
        Box::new(Dog { name: String::from("Fido") }),
    ];
    for pet in pets {
        println!("Hello {}!", pet.name());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn deriving_traits()  {
    let p1 = Player::default();
    let p2 = p1.clone();
    println!("Is {:?}\nequal to {:?}?\nThe answer is {}!", &p1, &p2,
             if p1 == p2 { "yes" } else { "no" });
}

trait Equals {
    fn equals(&self, other: &Self) -> bool;
    fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equals(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

fn default_method() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equals(&b));
}

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

// 다음에 대한 문법 슈가:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn trait_bounds() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
}

use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn impl_trait() {
    let x = get_x("foo");
    println!("{x}");
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn iterators() {
    let fib = Fibonacci { curr: 0, next: 1 };
    for (i, n) in fib.enumerate().take(5) {
        println!("fib({i}): {n}");
    }
}

fn fromIterator() {
    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes
        .into_iter()
        .map(|prime| prime * prime)
        .collect::<Vec<_>>();
}

fn from_and_into() {
    {
        let s = String::from("hello");
        let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
        let one = i16::from(true);
        let bigger = i32::from(123i16);
        println!("{s}, {addr}, {one}, {bigger}");
    }
    {
        let s: String = "hello".into();
        let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
        let one: i16 = true.into();
        let bigger: i32 = 123i16.into();
        println!("{s}, {addr}, {one}, {bigger}");
    }
}

use std::io::{BufRead, BufReader, Read, Result, Write};

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

fn fn_read() -> Result<()> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));

    let file = std::fs::File::open(std::env::current_exe()?)?;
    println!("lines in file: {}", count_lines(file));
    Ok(())
}

fn fn_write() -> Result<()> {
    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {:?}", buffer);
    Ok(())
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn drop_trait() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");
}

#[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Self("John Smith".into())
    }
}

fn default_trait() {
    let default_struct = Derived::default();
    println!("{default_struct:#?}");

    let almost_default_struct = Derived {
        y: "Y is set!".into(),
        ..Derived::default()
    };
    println!("{almost_default_struct:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
}

#[derive(Debug, Copy, Clone)]
struct Point2 { x: i32, y: i32 }

impl std::ops::Add for Point2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

fn add_mul() {
    let p1 = Point2 { x: 10, y: 20 };
    let p2 = Point2 { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}

fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn closure() {
    let add_3 = |x| x + 3;
    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("add_3: {}", apply_with_log(add_3, 20));

    let mut v = Vec::new();
    let mut accumulate = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));

    let multiply_sum = |x| x * v.into_iter().sum::<i32>();
    println!("multiply_sum: {}", apply_with_log(multiply_sum, 3));
}

fn make_greeter(prefix: String) -> impl Fn(&str) {
    return move |name| println!("{} {}", prefix, name)
}

fn closure2() {
    let hi = make_greeter("Hi".to_string());
    hi("there");
}

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>,
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn inner_width(&self) -> usize {
        std::cmp::max(
            self.title.chars().count(),
            self.widgets.iter().map(|w| w.width()).max().unwrap_or(0),
        )
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        self.label.len()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width: usize = self.width();
        write!(buffer, "{:^width$}", self.label);
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.label.len()+2
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        write!(buffer, "[ ");
        self.label.draw_into(buffer);
        write!(buffer, " ]");
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        self.inner_width()
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width: usize = self.width();
        writeln!(buffer, "+{:-<width$}+", "");
        writeln!(buffer, "|{:^width$}|", self.title);
        writeln!(buffer, "+{:=<width$}+", "");
        for widget in self.widgets.iter() {
            let mut sub_buffer = String::new();
            widget.draw_into(&mut sub_buffer);
            writeln!(buffer, "|{:^width$}|", sub_buffer);
        }
        writeln!(buffer, "+{:-<width$}+", "");
    }
}

fn gui_library() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}

// ========
// Rust GUI Demo 1.23
// ========

// This is a small text GUI demo.

// | Click me! |

fn line_alignment() {
    let width = 10;
    println!("left aligned:  |{:/<width$}|", "foo");
    println!("centered:      |{:/^width$}|", "foo");
    println!("right aligned: |{:/>width$}|", "foo");
}

// +--------------------------------+
// |       Rust GUI Demo 1.23       |
// +================================+
// | This is a small text GUI demo. |
// | +-----------+                  |
// | | Click me! |                  |
// | +-----------+                  |
// +--------------------------------+

// rust click event ??

pub fn week5() {
    generic_data_type();
    monomorphization();
    fn_trait();
    fn_trait_object();
    deriving_traits();
    default_method();
    trait_bounds();
    impl_trait();
    iterators();
    fromIterator();
    from_and_into();
    fn_read();
    fn_write();
    drop_trait();
    default_trait();
    add_mul();
    closure();
    closure2();

    println!();
    let width = 10;
    println!("left aligned:  |{:/<width$}|", "foo");
    println!("centered:      |{:/^width$}|", "foo");
    println!("right aligned: |{:/>width$}|", "foo");
    println!();

    gui_library();
    //line_alignment();
}