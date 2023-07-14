include!("week1.rs");

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn say_hello(name: String) {
    println!("Hello {name}")
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct Point2(i32, i32);

fn add(p1: &Point2, p2: &Point2) -> Point2 {
    Point2(p1.0 + p2.0, p1.1 + p2.1)
}

// 수명은 보통 '로 시작된다고 한다.
fn left_most<'a>(p1: &'a Point2, p2: &'a Point2) -> &'a Point2 {
    if p1.0 < p2.0 { p1 } else { p2 }
}

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

// fn erase(text: String) {
//     println!("Bye {text}!");
// }

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl Library {
    fn new() -> Library {
        //todo!("Initialize and return a 'Library' value")
        Library {
            books: Vec::new(),
        }
    }

    fn len(&self) -> usize {
       //todo!("Return the length of `self.books`")
       self.books.len()
    }

    fn is_empty(&self) -> bool {
      // todo!("Return `true` if `self.books` is empty")
      self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
       //todo!("Add a new book to `self.books`")
       self.books.push(book);
    }

    fn print_books(&self) {
       //todo!("Iterate over `self.books` and each book's title and year")
       for book in &self.books {
          println!("book:{}, year:{}", book.title, book.year); 
       }
    }

    fn oldest_book(&mut self) -> Option<&Book> {
       //todo!("Return a reference to the oldest book (if any)")
       self.books.sort_by(|a, b| a.year.cmp(&b.year));
       self.books.first()
    }
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);

    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");

    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");


    let s1: String = String::from("Rust");
    let s2: String = s1;
    println!("{s2}");

    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
    //println!("{name}");

    // mut에 따라 소유권 이동이 달라질까?

    let p1 = Point(3, 4);
    let p2: Point = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    {
        let p1 = Point2(3, 4);
        let p2 = Point2(10, 20);
        let p3 = add(&p1, &p2);
        println!("{p1:?} + {p2:?} = {p3:?}");
    }

    {
        let mut a: i32 = 10;
        let b: &i32 = &a;

        // {
        //     let c: &mut i32 = &mut a;
        //     *c = 20;
        // }

        println!("a: {a}");
        println!("b: {b}");
    }

    {
        let p1: Point2 = Point2(10, 10);
        let p2: Point2 = Point2(20, 20);
        let p3: &Point2 = left_most(&p1, &p2);
        println!("left-most point: {:?}", p3);
    }

    {
        let text = String::from("The quick brown fox jumps over the lazy dog.");
        let fox = Highlight(&text[4..19]);
        let dog = Highlight(&text[35..43]);
        //erase(text);
        println!("{fox:?}");
        println!("{dog:?}");
    }

    {
        let mut vec = vec![10, 20];
        vec.push(30);
        let midpoint = vec.len() / 2;
        println!("middle value: {}", vec[midpoint]);
        for item in &vec {
            println!("item: {item}");
        }
    }

    {
        let mut library: Library = Library::new();

        println!("The library is empty: library.is_empty() -> {}", library.is_empty());
        
        library.add_book(Book::new("Lord of the Rings", 1954));
        library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
        
        println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
        
        
        library.print_books();
        
        match library.oldest_book() {
           Some(book) => println!("The oldest book is {}", book.title),
           None => println!("The library is empty!"),
        }
        
        println!("The library has {} books", library.len());
        library.print_books();
    }

    {
        let v: Vec<i8> = vec![10, 20, 30];
        let mut iter = v.iter();
    
        println!("v[0]: {:?}", iter.next());
        println!("v[1]: {:?}", iter.next());
        println!("v[2]: {:?}", iter.next());
        println!("No more items: {:?}", iter.next());
    }

    {
        let v: Vec<i8> = vec![10, 20, 30];
        let mut iter = v.iter();

        let v0 = iter.next();
        println!("v0: {v0:?}");
    }

    {
        let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
        let mut iter = v.into_iter();

        let v0 = iter.next();
        println!("v0: {v0:?}");
    }
}