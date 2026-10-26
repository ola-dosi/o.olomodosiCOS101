use std::io::Write;

fn main() {

    let categories = "Lager,Stout,Non-Alcoholic
33 Export,Legend,Maltina
Desperados,Turbo King,Amstel Malta
Goldberg,Williams,Malta Gold
Gulder,,Fayrouz
Heineken,,
Star,,";

    let mut file = std::fs::File::create("categories_of_drinks.csv").expect("create failed");
    file.write_all("Nigerian Breweries Plc, incorporated in 1946\n"
           .as_bytes()).expect("Write failed");
    file.write_all(categories.as_bytes()).expect("Write failed");
    println!("\nData written to file.");
}
