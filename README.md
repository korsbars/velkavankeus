
The application is intended as a personal finance planning tool rather than financial advice.

There are numerous bugs that crash the user's computer. May or may not fix.
---

# Features

## Portfolio Management

* Add multiple loans
* Edit existing loans
* Remove loans
* Save portfolios as JSON
* Load previously saved portfolios

Each loan stores:

* Name
* Remaining balance
* Annual interest rate
* Minimum monthly payment

---

## Repayment Strategies

Currently supported:

* Minimum Only
* Avalanche
* Snowball

The design allows additional repayment strategies to be added with minimal changes by implementing the `PaymentStrategy` trait.

---

## Investment Comparison

The simulator compares two financial decisions:

### Scenario A

* Pay minimum payments
* Invest any available extra cash immediately
* Continue investing freed-up cash once loans are paid

### Scenario B

* Use an accelerated repayment strategy
* Invest only after loans have been repaid

The simulator reports:

* Time until debt free
* Total interest paid
* Interest saved
* Final investment value
* Net financial advantage

---

## Graphical User Interface

The GUI provides:

* Portfolio overview
* Loan management
* Investment comparison
* Interactive charts
* JSON save/load support

---

# Building

Clone the repository:

```bash
git clone <repository-url>
```

Build the project:

```bash
cargo build --release
```

Run:

```bash
cargo run
```

---

# Using the Program

## 1. Create a Portfolio

Click **Add Loan**.

Enter:

* Loan name
* Remaining balance
* Interest rate
* Minimum monthly payment

Repeat for each loan.

---

## 2. Save Your Portfolio

Press **Save**.

Choose a filename.

Your portfolio will be stored as a JSON file.

---

## 3. Load a Portfolio

Press **Load**.

Select a previously saved JSON portfolio.

---

## 4. Configure the Simulation

Choose:

* Extra monthly payment
* Expected annual investment return
* Repayment strategy

Examples:

* Avalanche
* Snowball

---

## 5. Run the Comparison

Results are generated automatically after selecting the inputs.

The program compares:

**Minimum payments + investing immediately**

versus

**Accelerated loan repayment + investing afterwards**

---

## 6. Review the Results

The comparison includes:

* Debt-free date
* Total interest
* Investment balance
* Net position
* Overall winner

Charts illustrate:

* Debt balance over time
* Investment growth
* Comparison between repayment approaches


---

# Adding New Repayment Strategies

Create a new strategy implementing the `PaymentStrategy` trait.

Example:

```rust
pub struct MyStrategy {
    pub extra_payment: f64,
}

impl PaymentStrategy for MyStrategy {
    // implementation...
}
```

Register it in the strategy registry so it becomes available throughout the application.

---

# Future Improvements

Planned features include:

* Additional repayment strategies
* Inflation modelling
* Taxes on investment gains
* Lump-sum payments
* Variable interest rates
* PDF and CSV export
* Richer charts and analytics

---

# Disclaimer

This software is intended for educational and financial planning purposes only.

Investment returns, inflation, and future market performance are uncertain. Always verify important financial decisions using professional advice where appropriate.

