trait Animal {
    fn speak(&self);
}

#[derive(Debug, Clone)]
struct Cat {
    name: String
}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow!"); 
    }
}

#[derive(Debug, Clone)]
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
    // string slice as name
    let jax = "jax";

    print_animal_name(&oreo);
    print_animal_name(jax);

    let cat = Box::new(Cat {name: peanut});
    let dog = Box::new(Dog {name: oreo});

    let dog2 = Dog {name: jax.to_owned()};

    print_dog(&dog);
    print_dog(&dog2);

    let animals: Vec<Box<dyn Animal>> = vec![cat.clone(), dog.clone()];
    let animals1: [Box<dyn Animal>; 2] = [cat, dog];

    animal_sounds(&animals);
    animal_sounds(&animals1);
}

// can accept args as string type, or string slice type,
// reference to a string could be converted to str slice implicitly
// so fun can accepts more type, prefer more implicit conversion type
fn print_animal_name(name: &str) {
    println!("{name}");
}

// ref to box containing dog, could be implicitly converted to ref to dog
fn print_dog(dog: &Dog) {
    println!("{:?}", dog);
}
// vec of box animal, could be implicitly converted to array of box animal
fn animal_sounds(animals: &[Box<dyn Animal>]) {
    for a in animals {
        a.speak()
    }
}