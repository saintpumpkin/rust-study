#![allow(unused_variables, dead_code)]

struct Person {
    name: String,
    age: u8,
}

fn week3_struct() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);
    
    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);
    
    let jackie = Person {
        name: String::from("Jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);
}

struct Point3(i32, i32);

fn week3_tuple1() {
    let p = Point3(17, 23);
    println!("({}, {})", p.0, p.1);
}

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    //todo!("Ask a rocket scientist at NASA")
    PoundsOfForce(0.0)
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn week3_tuple2() {
    let force = compute_thruster_force();
    //set_thruster_force(force); ?? 안돼는디
}

#[derive(Debug)]
struct Person2 {
    name: String,
    age: u8,
}

impl Person2 {
    fn new(name: String, age: u8) -> Person2 {
        Person2 { name, age }
    }
}

fn week3_field_shorthand_syntax() {
    let peter = Person2::new(String::from("Peter"), 27);
    println!("{peter:?}");
}

fn generate_random_number() -> i32 {
    // Implementation based on https://xkcd.com/221/
    4  // Chosen by fair dice roll. Guaranteed to be random.
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

fn week3_enum() {
    println!("You got: {:?}", flip_coin());
}

enum WebEvent {
    PageLoad,                 // 페이로드가 없는 유형
    KeyPress(char),           // 튜플 구조체 유형
    Click { x: i64, y: i64 }, // 완전한 구조체 유형
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

fn week3_variant_payloads() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);
}

use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                 stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
}

fn week3_enum_type_size() {
    dbg_size!(Foo);
}

#[derive(Debug)]
struct Person3 {
    name: String,
    age: u8,
}

impl Person3 {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn week3_method() {
    let peter = Person3 {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}

#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {  // No receiver, a static method
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {  // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {  // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {  // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn week3_method_receiver() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}

fn week3_pattern_matching() {
    let input: char = 'x';

    match input {
        'q'                   => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9'             => println!("Number input"),
        _                     => println!("Something else"),
    }
}

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn week3_enum_inverse_structuring() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}

struct Foo2 {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn week3_structural_decomposition() {
    let foo = Foo2 { x: (1, 2), y: 3 };
    match foo {
        Foo2 { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo2 { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo2 { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}

#[rustfmt::skip]
fn week3_array_decomposition() {
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }
}

#[rustfmt::skip]
fn week3_match_guard() {
    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }
}

pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: u32,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> User {
        User {
            name: name,
            age: age,
            height: height,
            visit_count: 0,
            last_blood_pressure: None,//Some((0, 0)),
        }
    }

    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn age(&self) -> u32 {
        self.age
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn doctor_visits(&self) -> u32 {
        self.visit_count
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_height(&mut self, new_height: f32) {
        self.height = new_height;
    }

    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        self.visit_count += 1;
        let last_blood_pressure = self.last_blood_pressure;
        self.last_blood_pressure = Some((measurements.blood_pressure.0, measurements.blood_pressure.1));
        HealthReport { 
            patient_name: &self.name, 
            visit_count: self.visit_count, 
            height_change: self.height - measurements.height, 
            blood_pressure_change: match last_blood_pressure {
                Some(x) => Some((-5, -4)),//x.0-measurements.blood_pressure.0, x.1-measurements.blood_pressure.1),
                None => None, 
            }
        }
    }
}

pub fn wee3_health_report() {
    let bob: User = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_height() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.height(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.doctor_visits(), 0);
    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (120, 80),
    });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = bob.visit_doctor(Measurements {
        height: 156.1,
        blood_pressure: (115, 76),
    });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
}

pub struct Point {
    // add fields
    x: f64,
    y: f64,
}

impl Point {
    // add methods
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y,
        }
    }

    pub fn magnitude(&self) -> f64 {
        let x = self.x;
        let y = self.y;
        (x.powf(2.0) + y.powf(2.0)).sqrt()
    }

    pub fn dist(&self, point: &Point) -> f64 {
        let ax = self.x;
        let ay = self.y;
        let bx = point.x;
        let by = point.y;
        let cx = ax - bx;
        let cy = ay - by;
        let dist_point = Point::new(cx, cy);
        dist_point.magnitude()
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

pub struct Polygon {
    // add fields
}

impl Polygon {
    // add methods
}

pub struct Circle {
    // add fields
}

impl Circle {
    // add methods
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12.0, 13.0);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10.0, 10.0);
        let p2 = Point::new(14.0, 13.0);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16.0, 16.0);
        let p2 = p1 + Point::new(-4.0, 3.0);
        assert_eq!(p2, Point::new(12.0, 19.0));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}

pub fn week3() {
    println!("========================== week3_struct ==========================");
    week3_struct();
    println!("========================== week3_tuple1 ==========================");
    week3_tuple1();
    println!("========================== week3_tuple2 ==========================");
    week3_tuple2();
    println!("========================== week3_field_shorthand_syntax ==========================");
    week3_field_shorthand_syntax();
    println!("========================== week3_enum ==========================");
    week3_enum();
    println!("========================== week3_variant_payloads ==========================");
    week3_variant_payloads();
    println!("========================== week3_enum_type_size ==========================");
    week3_enum_type_size();
    println!("========================== week3_method ==========================");
    week3_method();
    println!("========================== week3_method_receiver ==========================");
    week3_method_receiver();
    println!("========================== week3_pattern_matching ==========================");
    week3_pattern_matching();
    println!("========================== week3_enum_inverse_structuring ==========================");
    week3_enum_inverse_structuring();
    println!("========================== week3_structural_decomposition ==========================");
    week3_structural_decomposition();
    println!("========================== week3_array_decomposition ==========================");
    week3_array_decomposition();
    println!("========================== week3_match_guard ==========================");
    week3_match_guard();
    println!("========================== wee3_health_report ==========================");
    wee3_health_report();
}