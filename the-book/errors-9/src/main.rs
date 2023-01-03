pub mod letspanic;
pub mod letsrecover;
pub mod letspropagate;

//use crate::letspanic::panic_function;

fn main() {
    //println!("Hello, world! Let us panic a little bit:\n");

    // letspanic::panic_function();
    // letspanic::panic_from_other();

    //println!("Hello, world! Let us recover a little bit:\n");
    
    //letsrecover::open_file();
    //letsrecover::open_and_match_diff();
    //letsrecover::open_with_unwrap();
    //letsrecover::open_with_expect();

    println!("Hello, world! Let us propagate a little bit:\n");

    letspropagate::read_username_from_file();


    
}
