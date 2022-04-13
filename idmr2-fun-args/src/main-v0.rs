trait Animal {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat {
    name: String
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow!"); 
    }
}

#[derive(Debug)]
struct Dog {
    name: String
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof!");
    }
}

fn main() {
    let peanut = "peanut".to_owned();
    let oreo = "oreo".to_owned();

    print_animal_name(&oreo);

    let cat = Box::new(Cat {name: peanut});
    let dog = Box::new(Dog {name: oreo});

    print_dog(&dog);

    let animals: Vec<Box<dyn Animal>> = vec![cat, dog];

    animal_sounds(&animals);
}

fn print_animal_name(name: &String) {
    println!("{name}");
}

fn print_dog(dog: &Box<Dog>) {
    println!("{:?}", dog);
}

fn animal_sounds(animals: &Vec<Box<dyn Animal>>) {
    for a in animals {
        a.speak()
    }
}