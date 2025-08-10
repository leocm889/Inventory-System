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
        println!("Enter name:");

        let name = input_trimmed();

        println!("Enter quanity:");

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

        println!("Enter price:");

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

pub fn search_menu(inventory: &HashMap<Uuid, Item>) {
    loop {
        println!("Search by:");
        println!("1. ID");
        println!("2. Name");
        println!("3. Category");
        println!("4. Back to main menu");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter the ID to search:");
                let id_input = input_trimmed();
                match Uuid::parse_str(&id_input) {
                    Ok(id) => {
                        search_by_id(inventory, id);
                    }
                    Err(_) => println!("Invalid UUID format."),
                }
            }
            2 => {
                println!("Enter the name or part of the name to search:");
                let name_query = input_trimmed();
                search_by_name(inventory, &name_query);
            }
            3 => {
                let category = read_category();
                search_by_category(inventory, category);
            }
            4 => break,
            _ => println!("Invalid choice, try again."),
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

pub fn delete_item(inventory: &mut HashMap<Uuid, Item>) {
    println!("Please enter the id of the item you would like to delete:");
    let id_input = input_trimmed();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if inventory.remove(&id).is_some() {
                println!("Item deleted successfully");
            } else {
                println!("No item found with id: {id}");
            }
        }
        Err(_) => println!("Invalid UUID format."),
    }
}

pub fn update_item(inventory: &mut HashMap<Uuid, Item>) {
    println!("Please enter the id of the item you would like to update:");
    let id_input = input_trimmed();
    match Uuid::parse_str(&id_input) {
        Ok(id) => {
            if let Some(item) = inventory.get_mut(&id) {
                println!("Updating item: \n{item}");

                println!("Enter new name (or press Enter to keep '{}'):", item.name);
                let name = input_trimmed();
                if !name.is_empty() {
                    item.name = name;
                }

                println!(
                    "Enter new quantity (or press Enter to keep '{}'):",
                    item.quantity
                );
                let quantity_str = input_trimmed();
                if !quantity_str.is_empty() {
                    if let Ok(q) = quantity_str.parse::<u32>() {
                        item.quantity = q;
                    } else {
                        println!("Invalid quantity input, keeping old value.");
                    }
                }

                println!("Enter new price (or press Enter to keep '{}'):", item.price);
                let price_str = input_trimmed();
                if !price_str.is_empty() {
                    if let Ok(p) = price_str.parse::<f32>() {
                        item.price = p;
                    } else {
                        println!("Invalid price input, keeping old value.");
                    }
                }

                println!("Choose new category (or press Enter to keep current):");
                let new_category = read_category_optional(&item.category);
                if let Some(cat) = new_category {
                    item.category = cat;
                }

                println!("Item updated successfully.");
            } else {
                println!("No item found with ID: {id}");
            }
        }

        Err(_) => println!("Invalid UUID format."),
    }
}

fn read_category_optional(current: &Category) -> Option<Category> {
    loop {
        println!(
            "Choose category or press Enter to keep current ({:?}):",
            current
        );
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
                println!("Please enter a valid number");
                continue;
            }
        };

        match choice {
            1 => return Some(Category::Electronics),
            2 => return Some(Category::Food),
            3 => return Some(Category::Clothing),
            4 => return Some(Category::Other),
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn input_trimmed() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn read_category() -> Category {
    loop {
        println!("Choose category:");
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
            1 => return Category::Electronics,
            2 => return Category::Food,
            3 => return Category::Clothing,
            4 => return Category::Other,
            _ => {
                println!("Invalid choice, try again.");
                continue;
            }
        };
    }
}
