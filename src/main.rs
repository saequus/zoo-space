use crate::food::food::ToFeedAnimals;
mod food;
mod manage;
use std::env;
mod designators;

fn main () {
    let args: Vec<String> = env::args().collect(); 
    println!("Start of visit to Zoo Space");
    

    create_function!(temp);
    print_result!(1u32 + 1);

    show_block!({
        let x = 1u32;

        x * x + 2 - 1
    });

    let mut animal_name = String::from("[Animal name not provided]");
    if args.len() > 1 {
        animal_name = String::from(&args[1]); 
    }
    let example_an  = food::food::Animal {
        animal_type: food::food::AnimalType::Horse,
        name: animal_name,
    };

    println!("Animal name: {}", example_an.show_name());

    println!("End of visit Zoo Space");
}



