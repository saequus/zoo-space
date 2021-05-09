use crate::food::food::ToFeedAnimals;
mod food;

fn main () {
    println!("Start of visit to Zoo Space");

    let example_an  = food::food::Animal {
        animal_type: food::food::AnimalType::Horse,
        name: String::from("Elliot"),
    };

    println!("Animal name: {}", example_an.show_name());

    println!("End of visit Zoo Space");
}



