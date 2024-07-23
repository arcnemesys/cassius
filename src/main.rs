use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Customer {
    funds: f32,
    cart: HashMap<String, (CustomerItem, f32)>,
}

enum CheckoutPreference {
    Truncate,
    Cover,
}

#[derive(Debug, Clone)]
struct InventoryItem {
    name: String,
    price: f32,
    count: f32,
    tax: f32,
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

    pub fn add_item(&mut self, item: CustomerItem, quantity: f32) {
        self.cart
            .insert(item.product.name.clone(), (item, quantity));
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

fn line_item(product: &CustomerItem) -> Vec<String> {
    todo!()
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

    pub fn decrement_item_count(&mut self, item_name: &String, quantity: f32) {
        if self.items.get(item_name).unwrap().count <= (0.0 as f32) {
            self.remove_item(item_name);
        } else {
            self.items
                .entry(item_name.clone())
                .and_modify(|item| item.count -= quantity);
        }
    }
    pub fn increment_item_count(&mut self, item_name: &String, quantity: f32) {
        self.items
            .entry(item_name.clone())
            .and_modify(|item| item.count += quantity);
    }

    pub fn remove_item(&mut self, item_name: &String) {
        self.items.remove(item_name);
    }

    pub fn add_item(&mut self, item_name: String, price: f32, count: u32, tax: f32) {
        let store_item = InventoryItem {
            name: item_name.clone(),
            price,
            count: count as f32,
            tax,
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

    pub fn process_customers(&mut self, store: &mut Store) {
        let customers = self.aisle.customers.clone();

        let register = self.aisle.register.clone();

        for mut customer in customers {
            let cart = customer.cart.clone();
            let mut total = 0.0 as f32;
            let mut receipt = Vec::new();
            for item in cart {
                let (product_name, (purchase, count)) = item;

                let price = purchase.product.price;
                let tax = purchase.product.tax;

                let item_total = (price * count) * tax;
                store.decrement_item_count(&product_name, count);
                total += item_total;
                customer.pay(total);
                let line_item =
                    format!("{product_name}: {price}, x{count}, item_total: {item_total}.");

                receipt.push(line_item);
            }
        }

        // Loop through customers:
        // For each customer:
        // Start running total
        // Loop through items, grab item count, decrementing store inventory
        // For each item, check item count, multiply by that, then add tax and add to total
        // If customer funds too low, check preference on truncate purchase,
        // or adding funds, move accordingly.
        // If customer overpays, check that change is available in register
        // if register low, add funds to register
        // give customer change
        // print receipt.
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
