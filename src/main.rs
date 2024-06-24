use console::Term;
use tui::{backend::CrosstermBackend, Terminal};

fn main() {
    let terminal = Term::stdout();
    let backend = CrosstermBackend::new(terminal);
    let mut terminal = Terminal::new(backend);
}
