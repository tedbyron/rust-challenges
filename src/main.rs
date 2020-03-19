#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod _2020;

use _2020::camel_case::camel_case;

fn main() {
    println!("{:?}", camel_case("test STRING foR, camel case function."))
}
