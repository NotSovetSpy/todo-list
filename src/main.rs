use::console::Term;
use todo_list::{display_content, run, Status, TITLE, task_list_operation};

fn main() -> Result<(), std::io::Error>{
    let terminal = setup()?;
    
    loop {
        let mut tasks = task_list_operation::read_list();
        display_content(terminal.clone(), &tasks)?;
        let status = run(terminal.clone(), &mut tasks)?;
        
        if status == Status::Exit {
            break Ok(());
        }

        terminal.clear_screen().unwrap();
    }
}

fn setup() -> Result<Term, std::io::Error> {
    let terminal = Term::stdout();
    terminal.clear_screen()?;
    terminal.set_title(TITLE);
    Ok(terminal)
}
