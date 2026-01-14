use std::io::{self, Write};

pub fn prompt_selection(prompt: &str, max: usize) -> Option<usize> {
    loop {
        print!("{} (1-{}): ", prompt, max);
        if io::stdout().flush().is_err() {
            return None;
        }
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return None;
        }
        if let Ok(n) = input.trim().parse::<usize>() {
            if n >= 1 && n <= max {
                return Some(n - 1);
            }
        }
        eprintln!("Invalid selection, try again (or press Ctrl+C to quit)");
    }
}
