pub mod vectors;
pub mod strings;
pub mod hashmaps;

use crate::vectors::working_with_vectors;
use crate::strings::working_with_strings;
use crate::hashmaps::working_with_hashmaps;

fn main() {
    println!("Hello, world!");

    working_with_vectors();
    working_with_strings();
    working_with_hashmaps();

}
