use std::io;
use crate::sim::simulation::SimulationMode;
use crate::modeling::loan::Loan;
use crate::sim::strategy::PaymentStrategy;
use crate::sim::minimum::MinimumOnly;
use crate::sim::avalanche::Avalanche;
use crate::sim::snowball::Snowball;

pub fn read_strategy() -> Box<dyn PaymentStrategy> {
    loop {
        println!();
        println!("===== Repayment Strategy =====");
        println!("1. Minimum Only");
        println!("2. Avalanche");
        println!("3. Snowball");

        let choice = read_string("Choice:");

        match choice.as_str() {
            "1" => return Box::new(MinimumOnly),

            "2" => {
                let extra = read_f64("Extra payment per month:");
                return Box::new(Avalanche {
                    extra_payment: extra,
                });
            }

            "3" => {
                let extra = read_f64("Extra payment per month:");
                return Box::new(Snowball {
                    extra_payment: extra,
                });
            }

            _ => println!("Invalid choice."),
        }
    }
}

pub fn read_string(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

pub fn read_u64(prompt: &str) -> u64 {
    loop {
        let input = read_string(prompt);

        match input.parse::<u64>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid whole number."),
        }
    }
}

pub fn read_f64(prompt: &str) -> f64 {
    loop {
        let input = read_string(prompt);

        match input.parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

pub fn read_loan() -> Loan {
    println!();
    println!("=== Add Loan ===");

    let id = read_u64("ID:");
    let name = read_string("Name:");
    let amount = read_f64("Original amount:");
    let interest = read_f64("Interest rate (%):");
    let minimum = read_f64("Minimum payment:");

    Loan::new(
        id,
        name,
        amount,
        interest,
        minimum,
    )
}

pub fn read_simulation_mode() -> SimulationMode {
    loop {
        println!();
        println!("===== Simulation Output =====");
        println!("1. Summary");
        println!("2. Month by Month");

        let choice = read_string("Choice:");

        match choice.as_str() {
            "1" => return SimulationMode::Summary,
            "2" => return SimulationMode::MonthByMonth,
            _ => println!("Invalid choice."),
        }
    }
}

