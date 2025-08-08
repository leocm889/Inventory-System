use std::{collections::HashMap, io};
mod category;
mod item;
use crate::{category::Category, item::Item};

fn main() {
    let mut iventory: HashMap<Category, Vec<Item>> = HashMap::new();
    println!("Welcome to Inventory System!");

    loop {
        println!("What would you like to do?");
        println!("1. Add Item");
        println!("2. View Inventory");
        println!("3. Search Items");
        println!("4. Update Item");
        println!("5. Delete Item");
        println!("6. Exit");

        println!("Enter choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Faild to read line");

        let chioce: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        //match choice {
        //    //1 => {}
        //    //2 => {}
        //    //3 => {}
        //    //4 => {}
        //    //5 => {}
        //    6 => {
        //        println!("Goodbye! 👋");
        //    }
        //    _ => {
        //        println!("Invalid choice, try again.");
        //        continue;
        //    }
        //};
    }
}
