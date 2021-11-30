// On Windows platform, don't show a console when opening the app.
// #![windows_subsystem = "windows"]

mod backend;
mod state;
mod ui;
mod util;

fn main() {
    backend::cpu_id::init();

    //Initiate a simple logger that logs all events to the console

    //TODO: Use something more sophisticated
    simple_logger::SimpleLogger::new().init().unwrap();

    ui::run();
}
