use velkavankaus_simulaattori::ui::gui::window::GuiApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Loan Simulator",
        options,
        Box::new(|_cc| Ok(Box::new(GuiApp::new()))),
    )
}