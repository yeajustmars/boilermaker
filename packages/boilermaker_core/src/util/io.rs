use std::io;

use color_eyre::Result;

#[tracing::instrument]
pub fn prompt_confirm(prompt: &str, correct_phrase: &str) -> Result<bool> {
    print!("{prompt}");
    io::Write::flush(&mut io::stdout())?;

    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation)?;

    Ok(confirmation.trim() == correct_phrase)
}
