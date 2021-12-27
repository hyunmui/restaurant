use std::collections::HashMap;
// use std::{fmt, io};
use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1, 101);
}

fn function1() -> Result {
    Result::Ok(())
}

// fn function2() -> io::Result<()> {
//     io::Result::Ok(())
// }

fn function2() -> IoResult<()> {
    IoResult::Ok(())
}
