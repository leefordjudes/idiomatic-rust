#![allow(dead_code)]
use std::mem;

#[derive(Debug)]
enum User {
    Reader {name: String},
    Writer {name: String},
    Admin {name: String},
}

fn promote(u: &mut User) {
    use User::*;
    // clone() is replaced with mem::take()
    // mem::take => take the value from its place, and fill its place with default value to prevent panic.
    *u = match u {
        Reader { name } => Writer { name: mem::take(name) },
        Writer { name } => Admin { name: mem::take(name) },
        Admin { name: _ } => return,
    }

}
fn main() {
    let mut user = User::Reader {name: "Aplus".to_owned()};
    println!("{user:?}");

    promote(&mut user);
    println!("{user:?}");   // user promoted to writer

    promote(&mut user);
    println!("{user:?}");   // user promoted to admin

    promote(&mut user);
    println!("{user:?}");   // nothing happens
}
