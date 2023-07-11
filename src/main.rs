// .iter(), ().iter(), & -> *
// .rev(), ().rev()
// loop
// [[0,0,0];3];
// isize, usize
// assert_eq!, debug_assert_eq!
// loop
// as usize

fn star(n: i32) {
    for y in 0..n {
        for _x in 0..y {
            print!("*");
        }
        println!();
    }
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0,0,0]; 3];
    let iter: [usize; 3] = [0,1,2];
    loop {
        println!("in loop");
        break;
    }
    for y in iter.iter() {
    //for y in iter.iter() {
        //debug_assert_ne!(new_matrix.is_empty(), false);
        assert_eq!(new_matrix.is_empty(), false);
        for x in (0..new_matrix[0].len()).rev()/*0..3*/ {
            new_matrix[x][*y] = matrix[*y][x];
            print!(" {}", matrix[*y][x]);
        }
    }
    return new_matrix;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!(" {:?}", row);
    }
}

fn main() {

    let _tt = 0..1;

    star(5);

    let matrix: [[i32; 3]; 3] = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    // let matrix2: [[i32; 4]; 4] = [
    //     [101, 102, 103, 104], // <-- the comment makes rustfmt add a newline
    //     [201, 202, 203, 204],
    //     [301, 302, 303, 304],
    //     [401, 402, 403, 404],
    // ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}