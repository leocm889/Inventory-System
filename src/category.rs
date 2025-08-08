use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Category {
    Electronics,
    Food,
    Clothing,
    Other,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let label = match self {
            Category::Electronics => "Electronics",
            Category::Food => "Food",
            Category::Clothing => "Clothing",
            Category::Other => "Other",
        };
        write!(f, "{label}")
    }
}
