use::console::Term;
use todo_list::{display_content, run, Status, TITLE};

fn main() -> Result<(), std::io::Error>{
    let terminal = setup()?;
    
    loop {
        terminal.write_line(TITLE)?;
        terminal.write_line("")?;
        display_content(terminal.clone())?;
        let status = run(terminal.clone())?;
        
        if status == Status::Exit {
            break Ok(());
        }
    }
}

fn setup() -> Result<Term, std::io::Error> {
    let terminal = Term::stdout();
    terminal.clear_screen()?;
    terminal.set_title(TITLE);
    terminal.hide_cursor()?;
    Ok(terminal)
}
