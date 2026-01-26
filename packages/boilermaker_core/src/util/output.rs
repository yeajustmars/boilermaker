use tabled::{Table, Tabled, settings::Style};
use tracing::error;

pub fn print_table<I, T>(rows: I)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    let mut table = Table::new(rows);
    table.with(Style::psql());
    print!("\n{table}\n");
}

pub fn print_table_error<I, T>(rows: I, msg: Option<&str>)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    let mut table = Table::new(rows);
    table.with(Style::psql());

    match msg {
        Some(msg) => error!("{}\n\n{table}\n", msg),
        None => error!("\n{table}\n"),
    }
}
