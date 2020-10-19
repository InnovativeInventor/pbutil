extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std;

fn paste() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    std::print!("{}", ctx.get_contents().unwrap());
}

fn main() {
    paste();
}
