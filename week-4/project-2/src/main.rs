// Project II - Rust program to work out an employee's annual incentive
// from their experience and their age

use std::io;

fn main()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("\nEmployee Incentive Calculator");

    println!("\nIs the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    // trim takes off the newline, and to_lowercase means YES and Yes work too
    let experience = input1.trim().to_lowercase();

    println!("Enter the employee's age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    // start at 0 and let the checks below change it
    let mut incentive = 0;

    if experience == "yes"
    {
        // one if inside another: the outer one asks about experience,
        // this inner one only runs for experienced staff
        if age >= 40
        {
            incentive = 1560000;
        }
        else if age >= 30 && age <= 39
        {
            incentive = 1480000;
        }
        else if age < 28
        {
            incentive = 1300000;
        }
        // ages 28 and 29 are not in the question, so incentive stays 0
    }
    else
    {
        incentive = 100000;
    }

    println!("\nExperienced: {}", experience);
    println!("Age: {}", age);

    if incentive > 0
    {
        println!("Annual incentive: N{}", incentive);
    }
    else
    {
        println!("No incentive band was given for an experienced employee aged {}", age);
    }
}
