pub mod inventory;

pub struct Item {
    pub name: String,
    cost_price: f64,
}

impl Item {
    pub fn new(name: String, cost_price: f64) -> Self {
        Item { name, cost_price }
    }
    pub fn get_cost(&self) -> f64 {
        self.cost_price
    }
}
