extern crate clipboard;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use std;

fn paste() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    std::print!("{}", ctx.get_contents().unwrap());
}

fn main() {
    paste();
}
