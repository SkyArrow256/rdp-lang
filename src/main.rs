use std::fs;

fn main() {
    let text = fs::read_to_string("./helloworld").unwrap();
    rdp_lang::run(&text);
}
