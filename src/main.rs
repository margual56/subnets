#[macro_use]
extern crate text_io;

use subnets::ip::IP;

fn main() {
    // reads until a \n is encountered
    let line: &str = "192.168.0.15/16"; //read!("{}\n");

    match IP::from_str(line) {
        Err(e) => println!("{:}", e),
        Ok(i) => {
            println!("{}", i);
            println!("{:?}", i);
        }
    };
}
