use crate::food::food::ToFeedAnimals;
mod food;
mod manage;
use std::env;


fn main () {
    manage::index::vector_test(); 
    let args: Vec<String> = env::args().collect(); 
    if args.len() > 0 {
        let command = &args[1];
        let param1 = &args[2];

        let another_an = food::food::Animal {
            animal_type: food::food::AnimalType::Frog,
            name: String::from(param1),
        };

        manage::index::show_test();
        println!("Another animal name: {}", another_an.show_name());
    };
    println!("Start of visit to Zoo Space");

    let example_an  = food::food::Animal {
        animal_type: food::food::AnimalType::Horse,
        name: String::from("Elliot"),
    };

    println!("Animal name: {}", example_an.show_name());

    println!("End of visit Zoo Space");
}



