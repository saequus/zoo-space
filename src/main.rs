mod food;
mod manage;
use std::env;
mod designators;
mod logtree;
mod animal;

pub mod command_mod_types {
    pub const ANIMAL: &'static str = "animal";
    pub const MANAGE: &'static str = "manage";
    pub const LOGTREE: &'static str = "logtree";
}

fn main () {
    let args: Vec<String> = env::args().collect(); 
    println!("Start of visit to Zoo Space"); 

    if args.len() > 1 {  
        let command_mod_string = String::from(&args[1]);
        let command_mod = command_mod_string.as_str();

        match command_mod {
            command_mod_types::ANIMAL => {
                animal::index::run(); 
            }
            command_mod_types::MANAGE => {
                manage::index::run();
            }
            command_mod_types::LOGTREE => {
                logtree::index::run();
            }
            _ => panic!("Command parameter is not know")
        }
    }
        
    println!("End of visit Zoo Space");
}



