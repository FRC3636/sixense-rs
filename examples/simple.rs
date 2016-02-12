extern crate sixense;
use sixense::*;

fn main() {
    let sixense = Sixense::new();
    loop {
        println!("{:?}", sixense.all_newest_data());
    }
}
