# 📦 Rust CLI Inventory System

A simple command-line inventory management system built with Rust.
Allows you to add, view, search, update, and delete inventory items, each categorized for easy organization.

---

## ✨ Features

- Add new inventory items with auto-generated unique IDs (UUID).
- View all items in the inventory.
- Search items by:
  - Name (case-insensitive partial match)
  - Category
  - ID (UUID)
- Update existing items by their ID.
- Delete items by their ID.
- Categorize items into Electronics, Food, Clothing, and Other.

---

## 🧱 Project Structure

```text
├── src
│ ├── main.rs // Entry point: user menu and input loop
│ ├── item.rs // Item struct and related methods
│ ├── category.rs // Category enum for grouping items
```

---

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version recommended)  
  Install from [https://rustup.rs/](https://rustup.rs/)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/leocm889/Inventory-System.git
cd Inventory-System
```

2. Build the project

```bash
cargo build --release
```

3. Run the executable

```bash
cargo run
```

---

## 🧪 Example Usage

The CLI will present a menu with options:
1. Add Item
2. View Inventory
3. Search Items (by name, category, or ID)
4. Update Item (by ID)
5. Delete Item (by ID)
6. Exit

Follow the prompts to interact with your inventory.

---

## 📊 Data Model

Each `Item` contains:

- ID: UUID generated automatically
- Name: Name of the item
- Quantity: Stock quantity (u32)
- Price: Price per unit (f32)
- Category: One of Electronics, Food, Clothing, Other

---

## 🧠 Concepts Practiced

- **Structs & Enums**: Defining complex data types like `Item` and `Category`.
- **HashMap**: Using `HashMap` to store and manage inventory items with unique keys.
- **UUID**: Generating unique identifiers for items using the `uuid` crate.
- **Input Handling**: Reading and validating user input from the command line.
- **Pattern Matching**: Using `match` statements for control flow and input validation.
- **Traits & Display**: Implementing the `Display` trait for custom formatting of output.
- **Closures, Higher-Order Functions & Generics**: Using closures and generics to implement flexible and reusable search functionality.
- **Error Handling**: Handling parsing errors and invalid input gracefully.
- **Loops & Control Flow**: Managing interactive CLI loops and menu navigation.
- **Ownership & Borrowing**: Managing references and mutable borrows with collections.
- **Crate Usage**: Integrating third-party crates (`uuid`) to enhance functionality.

---

## ✍️ Author

- leocm889
