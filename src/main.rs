use std::{collections::HashMap, io};
mod category;
mod item;
use uuid::Uuid;

use crate::item::{add_item, delete_item, retrieve_all_items, search_menu, update_item, Item};

fn main() {
    let mut inventory: HashMap<Uuid, Item> = HashMap::new();
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

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => add_item(&mut inventory),
            2 => retrieve_all_items(&inventory),
            3 => search_menu(&inventory),
            4 => update_item(&mut inventory),
            5 => delete_item(&mut inventory),
            6 => {
                println!("Goodbye! 👋");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
                continue;
            }
        };
    }
}
