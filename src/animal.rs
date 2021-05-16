pub mod index {
    use crate::food::index::ToFeedAnimals; 

    pub fn run () {
        let animal_name = String::from("[Animal name not provided]"); 
        let example_an  = Animal {
            animal_type: AnimalType::Horse,
            name: animal_name,
        };
        println!("Animal name: {}", example_an.show_name());  
    }

    pub struct Animal {
        pub animal_type: AnimalType,
        pub name: String,
    }

    #[allow(dead_code)]
    pub enum AnimalType {
        Horse,
        Lion,
        Frog
    }

 
}
