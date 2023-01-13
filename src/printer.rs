#![allow(clippy::uninlined_format_args)]

use comfy_table::{Cell, Color, Table};

pub trait Printer {
    fn success(&mut self, target: &str, result: &str);
    fn error(&mut self, target: &str, result: &str);
    fn finish(&self);
}

pub struct TablePrinter {
    table: Table,
}

impl TablePrinter {
    pub fn new() -> Self {
        let mut table = Table::new();
        table.set_header(vec!["Target", "Result"]);
        TablePrinter { table }
    }
}

impl Printer for TablePrinter {
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

impl TextPrinter {
    pub fn new() -> Self {
        TextPrinter {}
    }
}

impl Printer for TextPrinter {
    fn success(&mut self, target: &str, result: &str) {
        println!("{}, {}", target, result);
    }

    fn error(&mut self, target: &str, result: &str) {
        println!("{}, {}", target, result);
    }

    fn finish(&self) {}
}
