use std::io;

fn main() {
    println!("================ RESTAURANT MENU ================");
    println!("P - Poundo Yam / Edinkaiko Soup   N3200");
    println!("F - Fried Rice & Chicken          N3000");
    println!("A - Amala & Ewedu Soup            N2500");
    println!("E - Eba & Egusi Soup              N2000");
    println!("W - White Rice & Stew             N2500");
    println!("===================================================");

    let mut food = String::new();
    println!("Enter food type:");
    io::stdin().read_line(&mut food).unwrap();
    let food = food.trim().to_uppercase();

    let mut quantity = String::new();
    println!("Enter quantity:");
    io::stdin().read_line(&mut quantity).unwrap();
    let quantity: i32 = quantity.trim().parse().unwrap();

    let price: i32;
    let food_name: &str;

    if food == "P" {
        price = 3200;
        food_name = "Poundo Yam / Edinkaiko Soup";
    } else if food == "F" {
        price = 3000;
        food_name = "Fried Rice & Chicken";
    } else if food == "A" {
        price = 2500;
        food_name = "Amala & Ewedu Soup";
    } else if food == "E" {
        price = 2000;
        food_name = "Eba & Egusi Soup";
    } else if food == "W" {
        price = 2500;
        food_name = "White Rice & Stew";
    } else {
        println!("Invalid food type.");
        return;
    }

    let mut total = price * quantity;

    println!("\nFood: {}", food_name);
    println!("Quantity: {}", quantity);
    println!("Total before discount: N{}", total);

    if total > 10000 {
        let discount = total * 5 / 100;
        total = total - discount;
        println!("5% discount: N{}", discount);
    }

    println!("Total charge: N{}", total);
}
