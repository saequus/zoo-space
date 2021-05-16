pub mod index {
    
    #[macro_export]
    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("You used {}", stringify!($func_name));
            }
        };
    }

    #[macro_export]
    macro_rules! print_result {
        ($expresion:expr) => {
            println!("{:?} = {:?}", stringify!($expresion), $expresion);
        };
    }

    #[macro_export]
    macro_rules! show_block {
        ($block:expr) => {
            println!("{:?} == {:?}", stringify!($block), $block);
        };
    }

    #[macro_export]
    macro_rules! wrapper {
        ($func_name:ident) => {
            println!("[WRAPPED] top");
            fn $func_name() {
                println!("[INNER] part: {:?} is called", stringify!($func_name));
            }
            println!("[WRAPPED] bottom");
        };
    }

}




