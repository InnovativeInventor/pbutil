extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use std;
use std::io::{self, Read};

fn main() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    let mut input_string = String::new();

    io::stdin().read_to_string(&mut input_string).expect("Not a string!");

    ctx.set_contents(input_string.to_owned()).unwrap();

    // Due to https://github.com/aweinstock314/rust-clipboard/issues/28
    println!("Copied to clipboard! Quit CLI after pasting.");
    loop {};
}
