use crate::category::Category;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
    io,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    id: u32,
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
    pub fn new(id: u32, name: String, quantity: u32, price: f32, category: Category) -> Self {
        Self {
            id,
            name,
            quantity,
            price,
            category,
        }
    }
}

pub fn add_item(inventory: &mut HashMap<Category, Vec<Item>>) {
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

        let quantity: f32 = match price.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };
    }
}

fn input_trimmed() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
