struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let quantity: u32 = 3;

    let hp = Laptop { brand: String::from("HP"), price: 650000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850000 };

    let hp_cost = hp.total_cost(quantity);
    let ibm_cost = ibm.total_cost(quantity);
    let toshiba_cost = toshiba.total_cost(quantity);
    let dell_cost = dell.total_cost(quantity);

    println!("{} x {} = {}", quantity, hp.brand, hp_cost);
    println!("{} x {} = {}", quantity, ibm.brand, ibm_cost);
    println!("{} x {} = {}", quantity, toshiba.brand, toshiba_cost);
    println!("{} x {} = {}", quantity, dell.brand, dell_cost);

    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;
    println!("Total cost for 3 laptops from each brand: {} naira", total);
}
