use std::io;

pub fn show_menu() -> String {
    println!();
    println!("=== Loan Simulator ===");
    println!("1. Add loan");
    println!("2. View portfolio");
    println!("3. Remove loan");
    println!("4. Run simulation");
    println!("5. Save Portfolio");
    println!("6. Load Portfolio");
    println!("7. Exit");
    println!();

    let mut input = String::new();

    print!("Choice: ");
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
