use crate::modeling::portfolio::Portfolio;
use crate::sim::simulation::{Simulation, SimulationMode};
use crate::storage::json;

use crate::ui::cli::{show_menu, input, output};

pub struct App {
    portfolio: Portfolio,
}

impl App {
    pub fn new() -> Self {
        Self {
            portfolio: Portfolio::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let choice = show_menu();

            match choice.as_str() {
                "1" => self.add_loan(),
                "2" => self.view_portfolio(),
                "3" => self.remove_loan(),
                "4" => self.run_simulation(),
                "5" => self.save_portfolio(),
                "6" => self.load_portfolio(),
                "7" => break,
                _ => println!("Invalid choice."),
            }
        }
    }

    fn add_loan(&mut self) {
        let loan = input::read_loan();
        self.portfolio.add_loan(loan);

        println!("Loan added successfully!");
    }

    fn view_portfolio(&self) {
        output::print_portfolio(&self.portfolio);
    }

    fn remove_loan(&mut self) {
        if self.portfolio.is_empty() {
            println!("Portfolio is empty.");
            return;
        }

        output::print_portfolio(&self.portfolio);

        let id = input::read_u64("Loan ID to remove:");

        if self.portfolio.remove_loan(id) {
            println!("Loan removed.");
        } else {
            println!("No loan with ID {} found.", id);
        }
    }

    fn run_simulation(&self) {
        if self.portfolio.is_empty() {
            println!("Portfolio is empty.");
            return;
        }

        let mode = input::read_simulation_mode();
        let strategy = input::read_strategy();

        let mut simulation = Simulation::new(
            self.portfolio.clone(),
            strategy.name().to_string(),
            strategy,
        );

        let result = simulation.run();

        match mode {
            SimulationMode::Summary => {
                output::print_simulation_result(&result);
            }
            SimulationMode::MonthByMonth => {
                output::print_monthly_history(&result);
            }
        }
    }

    fn save_portfolio(&self) {
        let filename = input::read_string("Filename:");

        match json::save_portfolio(&self.portfolio, &filename) {
            Ok(_) => println!("Portfolio saved."),
            Err(e) => println!("Failed to save: {}", e),
        }
    }

    fn load_portfolio(&mut self) {
        let filename = input::read_string("Filename:");

        match json::load_portfolio(&filename) {
            Ok(portfolio) => {
                self.portfolio = portfolio;
                println!("Portfolio loaded.");
            }
            Err(e) => println!("Failed to load: {}", e),
        }
    }

    pub fn portfolio(&self) -> &Portfolio {
        &self.portfolio
    }

    pub fn portfolio_mut(&mut self) -> &mut Portfolio {
        &mut self.portfolio
    }
}