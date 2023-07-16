// "" : &str, "".to_string() : String
// switch == match
// member function self exists 
// year: year, ???
// <'a> ???

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn week2_type_inference() {
    let x = 10;
    let y = 20;
    takes_u32(x);
    takes_i8(y);
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

fn week2_const() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}

fn week2_scopes_and_shadowing() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn week2_ownership() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    //println!("y: {}", p.1);
}

fn week2_move_semantics() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}

fn week2_moved_strings() {
    let s1: String = String::from("Rust");
    let s2: String = s1;
    println!("{s2}");
}

fn say_hello(name: String) {
    println!("Hello {name}")
}

fn week2_moves_in_function_calls() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
    //println!("{name}");
}

fn week2_copying_and_cloning() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");

    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

#[derive(Debug)]
struct Point2(i32, i32);

fn add(p1: &Point2, p2: &Point2) -> Point2 {
    Point2(p1.0 + p2.0, p1.1 + p2.1)
}

fn week2_borrowing() {
    let p1 = Point2(3, 4);
    let p2 = Point2(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}

fn week2_shared_and_unique_borrows() {
    let mut a: i32 = 10;
    //let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    //println!("b: {b}");
}

// 수명은 보통 '로 시작된다고 한다.
fn left_most<'a>(p1: &'a Point2, p2: &'a Point2) -> &'a Point2 {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn week2_lifetimes_in_function_calls() {
    let p1: Point2 = Point2(10, 10);
    let p2: Point2 = Point2(20, 20);
    let p3: &Point2 = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
}

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn erase(text: String) {
    println!("Bye {text}!");
}

fn week2_lifetimes_in_data_structures() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}

use std::collections::HashSet;

struct Library {
    books: Vec<Book>,
    borrowing: HashSet<String>,
}

struct Book {
    title: String,
    //title2: str,
    year: u16,
}

impl Book {
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year: year,
        }
        // Book {
        //     title: String::from(title),
        //     year,
        // }
    }

    fn print_info(&self) {
        println!(" - title: {}, year: {}", self.title, self.year);
    }
}

impl Library {
    fn new() -> Library {
        //todo!("Initialize and return a 'Library' value")
        Library {
            books: Vec::new(),
            borrowing: HashSet::new(),
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
          book.print_info(); 
       }
    }

    fn print_borrowing(&self) {
        for title in self.borrowing.iter() {
            println!(" - title: {}", title)
        }
    }

    fn oldest_book(&mut self) -> Option<&Book> {
       //todo!("Return a reference to the oldest book (if any)")
       self.books.sort_by(|a, b| a.year.cmp(&b.year));
       self.books.first()
    }

    // 책 검색
    fn find_book_index(&self, title: &String) -> usize {
        // for (i, book) in self.books.iter().enumerate() {
        //     if book.title == title {
        //         return i;
        //     }
        // }
        //return 0;
        let index = self.books.iter().position(|b| &b.title == title).unwrap_or(0);
        return index;
    }

    // 책 빌림
    fn borrow_book(&mut self, title: &String) -> Book {
        let index = self.find_book_index(title);
        self.borrowing.insert(String::from(title));
        self.books.swap_remove(index)
    }

    // 책 반납
    fn return_book(&mut self, book: Book) {
        self.borrowing.remove(&book.title);
        self.add_book(book);
    }

    fn test_todo(&self) {
        todo!("test todo!!!!!")
    }
}

fn week2_storing_books() {
    let mut library: Library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    
    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    
    library.print_books();

    {
        let num=11;
        match num{
            1=>println!("1"),
            2=>println!("2"),
            3=>println!("3"),
            4..=10=>println!("4 ~ 10"),
            11|20=>println!("11 or 20"),
            _=>println!("Rest of the number"),
        }
    }

    match library.oldest_book() {
       Some(book) => println!("The oldest book is {}", book.title),
       None => println!("The library is empty!"),
    }
    
    println!("The library has {} books", library.len());
    library.print_books();

    //library.test_todo();
}

fn week2_borrow_books() {
    let mut library: Library = Library::new();

    println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    library.print_books();
    
    let a: &str = "꼬시다꼬셔";
    let s = "Lord of the Rings";
    let title = s.to_string();
    let book = library.borrow_book(&title);
    println!("My Borrowing book");
    book.print_info();
    println!("Library");
    library.print_books();
    println!("Library borrowing");
    library.print_borrowing();
    println!();
    library.return_book(book);
    println!("Library");
    library.print_books();
    println!("Library borrowing");
    library.print_borrowing();
}


// fn main() {
    
//     // mut에 따라 소유권 이동이 달라질까?

//     let p1 = Point(3, 4);
//     let p2: Point = p1;
//     println!("p1: {p1:?}");
//     println!("p2: {p2:?}");

//     {
//         let p1 = Point2(3, 4);
//         let p2 = Point2(10, 20);
//         let p3 = add(&p1, &p2);
//         println!("{p1:?} + {p2:?} = {p3:?}");
//     }

//     {
//         let mut a: i32 = 10;
//         let b: &i32 = &a;

//         {
//             let c: &mut i32 = &mut a;
//             *c = 20;
//         }

//         println!("a: {a}");
//         println!("b: {b}");
//     }

//     {
//         let p1: Point2 = Point2(10, 10);
//         let p2: Point2 = Point2(20, 20);
//         let p3: &Point2 = left_most(&p1, &p2);
//         println!("left-most point: {:?}", p3);
//     }

//     {
//         let text = String::from("The quick brown fox jumps over the lazy dog.");
//         let fox = Highlight(&text[4..19]);
//         let dog = Highlight(&text[35..43]);
//         //erase(text);
//         println!("{fox:?}");
//         println!("{dog:?}");
//     }

//     {
//         let vv = [1, 2];
//         let mut vec = vec![10, 20];
//         vec.push(30);
//         let midpoint = vec.len() / 2;
//         println!("middle value: {}", vec[midpoint]);
//         for item in &vec {
//             println!("item: {item}");
//         }
//     }

//     {
//         let mut library: Library = Library::new();

//         println!("The library is empty: library.is_empty() -> {}", library.is_empty());
        
//         library.add_book(Book::new("Lord of the Rings", 1954));
//         library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
        
//         println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
        
        
//         library.print_books();

//         {
//             let num=11;
//             match num{
//                 1=>println!("1"),
//                 2=>println!("2"),
//                 3=>println!("3"),
//                 4..=10=>println!("4 ~ 10"),
//                 11|20=>println!("11 or 20"),
//                 _=>println!("Rest of the number"),
//             }
//         }

//         match library.oldest_book() {
//            Some(book) => println!("The oldest book is {}", book.title),
//            None => println!("The library is empty!"),
//         }
        
//         println!("The library has {} books", library.len());
//         library.print_books();

//         let a: &str = "꼬시다꼬셔";
//         let s = "Lord of the Rings";
//         let title = s.to_string();
//         let book = library.borrow_book(&title);
//         println!("My Borrowing book");
//         book.print_info();
//         println!("Library");
//         library.print_books();
//         println!("Library borrowing");
//         library.print_borrowing();
//         println!();
//         library.return_book(book);
//         println!("Library");
//         library.print_books();
//         println!("Library borrowing");
//         library.print_borrowing();
//     }

//     {
//         let v: Vec<i8> = vec![10, 20, 30];
//         let mut iter = v.iter();
    
//         println!("v[0]: {:?}", iter.next());
//         println!("v[1]: {:?}", iter.next());
//         println!("v[2]: {:?}", iter.next());
//         println!("No more items: {:?}", iter.next());
//     }

//     {
//         let v: Vec<i8> = vec![10, 20, 30];
//         let mut iter = v.iter();

//         let v0 = iter.next();
//         println!("v0: {v0:?}");
//     }

//     {
//         let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
//         let mut iter = v.into_iter();

//         let v0 = iter.next();
//         println!("v0: {v0:?}");
//     }

//     {
//         let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

//         // let mut iter = v.into_iter();

//         // let v0 = iter.next();
//         // println!("v0: {v0:?}");
        
//         for word in &v {
//             println!("word: {word}");
//         }

//         for word in v {
//             println!("word: {word}");
//         }
//     }
// }