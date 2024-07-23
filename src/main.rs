use std::collections::HashMap;

struct Customer {
    funds: f32,
    items: HashMap<String, Item>,
}

struct Item {
    name: String,
    price: f32,
    count: f32,
}

impl Customer {
    pub fn new(funds: f32) -> Self {
        Self {
            funds,
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.insert(item.name.clone(), item);
    }
    pub fn remove_item(&mut self, item_name: String) {
        if self.items.contains_key(&item_name) {
            self.items.remove(&item_name);
        }
    }

    pub fn pay(&mut self, total: f32) -> Vec<String> {
        let mut receipt = Vec::new();

        if self.funds >= total {
            self.funds -= total;

            for i in self.items.keys() {
                let price = self.items.get(i).unwrap().price;
                let count = self.items.get(i).unwrap().count;
                let line_item = format!(r"{}: {}, x{} \n", i, price, count);
                receipt.push(line_item);
            }
        } else {
            let remaining = total - self.funds;
            println!(
                "Not enough funds available for purchase, funds: {}, total: {}, remaining: {}",
                &self.funds, total, remaining
            );
        }

        receipt
    }
}

struct StoreInventory {
    items: HashMap<String, Item>,
}

impl StoreInventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn decrement_item_count(&mut self, item_name: String) {
        if self.items.get(&item_name).unwrap().count <= (0.0 as f32) {
            self.remove_item(item_name);
        } else {
            self.items
                .entry(item_name)
                .and_modify(|item| item.count -= 1.0);
        }
    }
    pub fn increment_item_count(&mut self, item_name: String) {
        self.items
            .entry(item_name)
            .and_modify(|item| item.count += 1.0);
    }

    pub fn remove_item(&mut self, item_name: String) {
        self.items.remove(&item_name);
    }

    pub fn add_item(&mut self, item_name: String, price: f32, count: u32) {
        let store_item = Item {
            name: item_name.clone(),
            price,
            count: count as f32,
        };

        self.items.insert(item_name, store_item);
    }
}

struct Cashier {
    register_no: u32,
    aisle_no: u32,
}
struct Register {
    number: u32,
    funds: f32,
}

struct Aisle {
    cashier: Cashier,
    customers: Vec<Customer>,
    register: Register,
}

fn main() {
    println!("Hello, world!");
}
