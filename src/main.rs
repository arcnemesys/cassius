use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Customer {
    funds: f32,
    cart: HashMap<String, CustomerItem>,
}

#[derive(Debug, Clone)]
struct InventoryItem {
    name: String,
    price: f32,
    count: f32,
}
#[derive(Debug, Clone)]
struct CustomerItem {
    product: InventoryItem,
    discarded: bool,
}

impl Customer {
    pub fn new(funds: f32) -> Self {
        Self {
            funds,
            cart: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: CustomerItem) {
        self.cart.insert(item.product.name.clone(), item);
    }
    pub fn remove_item(&mut self, item_name: String) {
        if self.cart.contains_key(&item_name) {
            self.cart.remove(&item_name);
        }
    }

    pub fn pay(&mut self, total: f32) -> Vec<String> {
        let mut receipt = Vec::new();

        if self.funds >= total {
            self.funds -= total;

            for i in self.cart.keys() {
                let price = self.cart.get(i).unwrap().product.price;
                let count = self.cart.get(i).unwrap().product.count;
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

#[derive(Debug, Clone)]
struct Store {
    items: HashMap<String, InventoryItem>,
    cashiers: Vec<Cashier>,
    aisles: Vec<Aisle>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            cashiers: Vec::new(),
            aisles: Vec::new(),
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
        let store_item = InventoryItem {
            name: item_name.clone(),
            price,
            count: count as f32,
        };

        self.items.insert(item_name, store_item);
    }
}

#[derive(Debug, Clone)]
struct Cashier {
    register_no: u32,
    aisle: Aisle,
}

impl Cashier {
    pub fn new() -> Self {
        Self {
            register_no: 0,
            aisle: Aisle::new(),
        }
    }

    pub fn process_customers(&mut self) {
        let customers = self.aisle.customers.clone();

        let register = self.aisle.register.clone();

        // Loop through customers:
        // For each customer:
        // Loop through items, incrementing store inventory
    }
}

#[derive(Debug, Clone)]
struct Register {
    number: u32,
    funds: f32,
}

impl Register {
    pub fn new() -> Self {
        Self {
            number: 0,
            funds: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct Aisle {
    customers: Vec<Customer>,
    register: Register,
}

impl Aisle {
    pub fn new() -> Self {
        Self {
            customers: Vec::new(),
            register: Register::new(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
