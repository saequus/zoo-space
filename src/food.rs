pub mod index { 
    use crate::animal::index::Animal;
    use crate::animal::index::AnimalType;

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

    #[allow(dead_code)]
    enum AnimalFoodType {
        Grass,
        Banana,
        Meat,
        Insects
    }

}
