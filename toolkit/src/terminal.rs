use crossterm::{
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn cleanup() {
    execute!(std::io::stdout(), LeaveAlternateScreen).unwrap();
    execute!(std::io::stderr(), LeaveAlternateScreen).unwrap();
    std::process::exit(0);
}

pub fn setup() {
    execute!(std::io::stdout(), EnterAlternateScreen).unwrap();
    execute!(std::io::stderr(), EnterAlternateScreen).unwrap();
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    execute!(std::io::stderr(), Clear(ClearType::All)).unwrap();
    ctrlc::set_handler(cleanup).unwrap();
}
