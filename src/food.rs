pub mod food {
    pub trait ToFeedAnimals {
        fn food_for_animal(animal: AnimalType) -> AnimalFoodType; 
        fn show_name(&self) -> String; 
    }

    impl ToFeedAnimals for Animal {
       fn food_for_animal(animal: AnimalType) -> AnimalFoodType {
            match animal {
                AnimalType::Horse => AnimalFoodType::Grass,
                AnimalType::Lion => AnimalFoodType::Meat,
                AnimalType::Frog => AnimalFoodType::Insects,
            }
        } 

        fn show_name(&self) -> String {
            format!("{}", self.name)
        }
    }

    pub struct Animal {
        pub animal_type: AnimalType,
        pub name: String,
    }

    enum AnimalFoodType {
        Grass,
        Banana,
        Meat,
        Insects
    }

    pub enum AnimalType {
        Horse,
        Lion,
        Frog
    }

}
