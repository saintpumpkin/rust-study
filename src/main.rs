include!("week1.rs");
include!("week2.rs");

fn main() {
    println!("========================== week2_type_inference ==========================");
    week2_type_inference();
    println!("========================== week2_const ==========================");
    week2_const();
    println!("========================== week2_scopes_and_shadowing ==========================");
    week2_scopes_and_shadowing();
    println!("========================== week2_ownership ==========================");
    week2_ownership();
    println!("========================== week2_move_semantics ==========================");
    week2_move_semantics();
    println!("========================== week2_moved_strings ==========================");
    week2_moved_strings();
    println!("========================== week2_moves_in_function_calls ==========================");
    week2_moves_in_function_calls();
    println!("========================== week2_copying_and_cloning ==========================");
    week2_copying_and_cloning();
    println!("========================== week2_borrowing ==========================");
    week2_borrowing();
    println!("========================== week2_shared_and_unique_borrows ==========================");
    week2_shared_and_unique_borrows();
    println!("========================== week2_lifetimes_in_function_calls ==========================");
    week2_lifetimes_in_function_calls();
    println!("========================== week2_lifetimes_in_data_structures ==========================");
    week2_lifetimes_in_data_structures();
    println!("========================== week2_storing_books ==========================");
    week2_storing_books();
    println!("========================== week2_borrow_books ==========================");
    week2_borrow_books();
}