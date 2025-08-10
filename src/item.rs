use uuid::Uuid;

use crate::category::Category;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    io,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    id: Uuid,
    name: String,
    quantity: u32,
    price: f32,
    category: Category,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "ID: {}\nName: {}\nQuantity: {}\nPrice: {:.2}\nCategory: {:?}\n",
            self.id, self.name, self.quantity, self.price, self.category
        )
    }
}

impl Item {
    pub fn new(name: String, quantity: u32, price: f32, category: Category) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            quantity,
            price,
            category,
        }
    }
}

pub fn add_item(inventory: &mut HashMap<Uuid, Item>) {
    loop {
        println!("Enter name: ");

        let name = input_trimmed();

        let mut quantity = String::new();

        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read line");

        let quantity: u32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let mut price = String::new();

        io::stdin()
            .read_line(&mut price)
            .expect("Failed to read line");

        let price: f32 = match price.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        let category = read_category();

        let item = Item::new(name, quantity, price, category);
        inventory.insert(item.id, item);
        println!("Item added successfully.");
        break;
    }
}

pub fn retrieve_all_items(inventory: &HashMap<Uuid, Item>) {
    if inventory.is_empty() {
        println!("No items in the iventory list yet.");
        return;
    }

    for item in inventory.values() {
        println!("{item}");
    }
}

pub fn search_items<F>(inventory: &HashMap<Uuid, Item>, predicate: F)
where
    F: Fn(&Item) -> bool,
{
    let results: Vec<&Item> = inventory.values().filter(|item| predicate(item)).collect();

    if results.is_empty() {
        println!("No items found matching the criteria.");
    } else {
        println!("Found {} item(s):", results.len());
        for item in results {
            println!("{item}");
        }
    }
}

pub fn search_by_name(inventory: &HashMap<Uuid, Item>, query: &str) {
    let query_lower = query.to_lowercase();
    search_items(inventory, |item| {
        item.name.to_lowercase().contains(&query_lower)
    });
}

pub fn search_by_category(inventory: &HashMap<Uuid, Item>, category: Category) {
    search_items(inventory, |item| item.category == category);
}

pub fn search_by_id(inventory: &HashMap<Uuid, Item>, id: Uuid) {
    match inventory.get(&id) {
        Some(item) => println!("Item found:\n{item}"),
        None => println!("No item found with ID: {id}"),
    }
}
//pub fn search_item_by_id(inventory: &HashMap<Uuid, Item>) {
//    println!("Enter the id to search for:");
//    let id_str = input_trimmed();
//
//    match Uuid::parse_str(&id_str) {
//        Ok(id) => match inventory.get(&id) {
//            Some(item) => println!("Found item:\n{}", item),
//            None => println!("No item found with ID: {}", id),
//        },
//        Err(_) => println!("Invalid UUID format."),
//    }
//}
//
//pub fn search_item_by_name(inventory: &HashMap<Uuid, Item>) {
//    println!("Enter the name or part of the name to search for:");
//    let query = input_trimmed().to_lowercase();
//
//    let mut results: Vec<&Item> = inventory
//        .values()
//        .filter(|item| item.name.to_lowercase().contains(&query))
//        .collect();
//
//    if results.is_empty() {
//        println!("No items found matching '{query}'.");
//    } else {
//        println!("Found {} item(s):", results.len());
//        for item in results {
//            println!("{item}");
//        }
//    }
//}

fn input_trimmed() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_category() -> Category {
    loop {
        println!("Choose your category within these options: ");
        println!("1. Electronics");
        println!("2. Food");
        println!("3. Clothing");
        println!("4. Other");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match choice {
            1 => Category::Electronics,
            2 => Category::Food,
            3 => Category::Clothing,
            4 => Category::Other,
            _ => {
                println!("Invalid choice, try again.");
                continue;
            }
        };
    }
}
