// Project I - Rust program to find the roots of a quadratic equation
// of the form ax^2 + bx + c = 0

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("\nQuadratic Equation Solver");
    println!("For an equation of the form ax*x + bx + c = 0\n");

    println!("Enter a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    // if a is 0 there is no x*x term at all, so it is not a quadratic
    if a == 0.0
    {
        println!("\na cannot be zero, that is not a quadratic equation");
    }
    else
    {
        // the discriminant decides how many roots there are
        let d:f32 = b * b - 4.0 * a * c;
        println!("\nDiscriminant = {}", d);

        if d > 0.0
        {
            let root1:f32 = (-b + d.sqrt()) / (2.0 * a);
            let root2:f32 = (-b - d.sqrt()) / (2.0 * a);
            println!("There are two distinct roots");
            println!("Root 1 = {:.2}", root1);
            println!("Root 2 = {:.2}", root2);
        }
        else if d == 0.0
        {
            // sqrt(0) is 0 so the two roots collapse into the same one
            let root:f32 = -b / (2.0 * a);
            println!("There is exactly one real root");
            println!("Root = {:.2}", root);
        }
        else
        {
            println!("There are no real roots");
        }
    }
}
