#[derive(Debug)]
enum User {
    Reader {name: String},
    Writer {name: String},
    Admin {name: String},
}

fn promote(u: &mut User) {
    use User::*;

    *u = match u {
        Reader { name } => Writer { name: name.clone() },
        Writer { name } => Admin { name: name.clone() },
        Admin { name: _ } => return,
    }

}
fn main() {
    println!("Hello, world!");
}
