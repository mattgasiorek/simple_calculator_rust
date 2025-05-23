/*
Project Name: Simple Calculator
Author: Matthew Gasiorek
Created On: 2025-05-21
Last Modified: 2025-05-22
Description: This is a basic calculator with +,-,*,/ operators.
Version: 1.0
Contact: matthew.gasiorek.coding@gmail.com
*/

use std::io;

fn main() {
    println!("\nWelcome to the Rust Calculator! (type quit at anytime to close the application)");

    loop {
        //start the loop
        //ask what the first number is
        println!("\nPlease enter the first number:");

        let mut num1_str = String::new(); //declare a mutable variable called num1
        io::stdin()
            .read_line(&mut num1_str) //read input from the user and pass a mutable reference to num1
            .expect("Failed to read line. Something went wrong"); //freak out if something goes wrong

        //the reason why we trim() is because Rust interprets the enter key press and then stores it with num1. So if I were to type, '5(enter key)', that's how it interprets it originally.
        let num1: f64 = match num1_str.trim().parse() {
            //convert num1_str to a floating point value
            Ok(num) => num, //If the parsing is successful (`Ok` variant), we get the number.
            Err(_) => {
                //If parsing fails (`Err` variant), it means the user didn't enter a valid number.
                if num1_str.trim().eq_ignore_ascii_case("quit") {
                    //if parsing fails due to "quit" being entered, exit the loop and close the program. If not this, then return invalid input and ask again.
                    println!("Exiting calculator.");
                    break;
                }
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };
        //ask the operator
        println!("Please enter the operator (+, -, * /):");
        let mut operator_str = String::new(); //declare a mutable variable called operator_str
        io::stdin()
            .read_line(&mut operator_str) //read input from the user and pass a mutable reference to operator_str
            .expect("Failed to read line. Something went wrong"); //freak out if something goes wrong

        let operator = operator_str.trim().to_string();

        match operator.as_str() {
            //think of match like IFS() in excel with multiple conditions
            "+" | "-" | "*" | "/" => {}
            _ => {
                if operator.eq_ignore_ascii_case("quit") {
                    //if parsing fails due to "quit" being entered, exit the loop and close the program. If not this, then return invalid input and ask again.
                    println!("Exiting calculator.");
                    break;
                }
                println!("Invalid operator. Please enter a valid operator (+, -, *, /.");
                continue;
            }
        }
        //ask what the second number is
        println!("\nPlease enter the second number:");

        let mut num2_str = String::new(); //declare a mutable variable called num1
        io::stdin()
            .read_line(&mut num2_str) //read input from the user and pass a mutable reference to num1
            .expect("Failed to read line. Something went wrong"); //freak out if something goes wrong

        let num2: f64 = match num2_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if num2_str.trim().eq_ignore_ascii_case("quit") {
                    println!("Exiting calculator.");
                    break;
                }
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };
        //In Rust, you have to account for all different options when using match, including unexpected inputs.
        let result = match operator.as_str() {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                // checks if we input zero, only if we put in "/"
                if num2 == 0.0 {
                    println!("Error: Division by zero is not allowed.");
                    continue; // Go back to the start of the loop
                }
                num1 / num2
            }
            _ => {
                println!("Unexpected error with operator. Starting over.");
                continue; // Go back to the start of the loop
            }
        };
        println!("Result: {} {} {} = {}", num1, operator, num2, result);
    } //this is the ending of the main loop
}
