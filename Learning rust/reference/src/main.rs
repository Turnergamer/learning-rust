use std::io;



struct Dog {
    name: String,
    age: u8,
    color: String,
}

fn print_dog_age(dog: &Dog) {
    println!("{} is {} years old and has {} colored fur!", dog.name, dog.age, dog.color);
}







fn get_dog_names(dogs: &[Dog]) {
    for dog in dogs {
        println!("{}", dog.name);
    }
}

fn main() {
    let cody = Dog {
        name: String::from("Cody"),
        age: 17,
        color: String::from("Black"),
    };
    let murphey = Dog {
        name: String::from("Murphey"),
        age: 21,
        color: String::from("Black"),
    };
    let turner = Dog {
        name: String::from("Turner"),
        age: 6,
        color: String::from("Orange"),
    };
    let mut name = String::new();

    let dogs = vec![cody, murphey, turner];


    get_dog_names(&dogs);

    println!("Choose from any Dogs");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();

    let dog = dogs.iter().find(|d| d.name == name);

    match dog {
        Some(dog) => print_dog_age(dog),
        None => println!("No dog with name {} found", name),}
}