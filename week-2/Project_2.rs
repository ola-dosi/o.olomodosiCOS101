fn main (){
	//sales from the table
	let toshiba: f64 = 450000.00;
	let mac: f64 = 1500000.00;
	let hp: f64 = 750000.00;
	let dell: f64 = 2850000.00;
	let acer: f64 = 250000.00;

	let number_of_items: f64 = 5.0; 
	
	//sum of all sales
	let mut sum: f64 =0.0;
	sum = sum + toshiba + mac + hp + dell + acer;

	//average sales
	let average = sum / number_of_items;

	println!("the total sales sum is {}",sum );
	println!("the average sale is {}",average );
	
}