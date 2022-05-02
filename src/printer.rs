use comfy_table::{Cell, Color, Table};

pub trait Printer {
    fn new() -> Self
    where
        Self: Sized;
    fn success(&mut self, target: &str, result: &str);
    fn error(&mut self, target: &str, result: &str);
    fn finish(&self);
}

pub struct TablePrinter {
    table: Table,
}

impl Printer for TablePrinter {
    fn new() -> Self {
        let mut table = Table::new();
        table.set_header(vec!["Target", "Result"]);
        TablePrinter { table }
    }

    fn success(&mut self, target: &str, result: &str) {
        self.table.add_row(vec![target, result]);
    }

    fn error(&mut self, target: &str, result: &str) {
        self.table
            .add_row(vec![Cell::new(target), Cell::new(result).bg(Color::Red)]);
    }

    fn finish(&self) {
        println!("{}", self.table);
    }
}

pub struct TextPrinter {}

impl Printer for TextPrinter {
    fn new() -> Self {
        TextPrinter {}
    }

    fn success(&mut self, target: &str, result: &str) {
        println!("{}, {}", target, result);
    }

    fn error(&mut self, target: &str, result: &str) {
        println!("{}, {}", target, result);
    }

    fn finish(&self) {}
}
