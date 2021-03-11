use curl::easy::Easy;
use std::io::{stdout,Write};
fn main() {
    //println!("Hello, world!");
    let mut easy = Easy::new();
    easy.url("eth0.me").unwrap();

    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    easy.perform().unwrap();

}
