use crate::modeling::loan::Loan;
use crate::modeling::portfolio::Portfolio;
use crate::sim::simulation::SimulationResult;

pub fn print_loan(loan: &Loan) {
    println!("-------------------------");
    println!("ID: {}", loan.id());
    println!("Name: {}", loan.name());
    println!("Original Amount: {:.2}", loan.original_amount());
    println!("Remaining Balance: {:.2}", loan.remaining_balance());
    println!("Interest Rate: {:.2}%", loan.interest_rate());
    println!("Minimum Payment: {:.2}", loan.minimum_payment());
    println!("Months Elapsed: {}", loan.months_elapsed());
    println!("Interest Paid: {:.2}", loan.total_interest_paid());

    if loan.is_paid_off() {
        println!("Status: Paid Off");
    } else {
        println!("Status: Active");
    }
}

pub fn print_portfolio(portfolio: &Portfolio) {
    println!();
    println!("===== Portfolio =====");

    if portfolio.is_empty() {
        println!("No loans.");
    } else {
        for loan in portfolio.loans() {
            print_loan(loan);
        }
    }

    println!("-------------------------");
    println!("Loan count: {}", portfolio.loan_count());
    println!(
        "Total Original: {:.2}",
        portfolio.total_original_amount()
    );
    println!(
        "Remaining Balance: {:.2}",
        portfolio.total_balance()
    );
    println!(
        "Interest Paid: {:.2}",
        portfolio.total_interest_paid()
    );
}

pub fn print_simulation_result(result: &SimulationResult) {
    println!();
    println!("===== Simulation Complete =====");
    println!("Months: {}", result.months());
    println!("Interest Paid: {:.2}", result.total_interest_paid());
    println!("Total Paid: {:.2}", result.total_paid());
}

pub fn print_monthly_history(result: &SimulationResult) {
    println!();
    println!("===== Monthly History =====");

    for snapshot in result.history() {
        println!("Month {}", snapshot.month());

        print_portfolio(snapshot.portfolio());

        println!("-------------------------");
    }

    print_simulation_result(result);
}