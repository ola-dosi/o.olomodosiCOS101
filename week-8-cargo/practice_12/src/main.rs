fn main() {
    // mutable array
    let mut colors = ["red", "green", "yellow", "white"];

    println!("\nOriginal array = {:?}", colors);

    // mutable slice
    let sliced_colors = &mut colors[1..3];

    println!("First slice = {:?}", sliced_colors);

    // change the value of the slice at the second position (index 1 of the slice)
    sliced_colors[1] = "purple";

    println!("Changed slice = {:?}", sliced_colors);

    // the slice is not a copy, so the original array changed too
    println!("Original array after change = {:?}", colors);
}
