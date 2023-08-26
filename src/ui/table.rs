use anyhow::{anyhow, Result};
use ellipse::Ellipse;
use std::fmt::{Display, Write};

pub struct Table {
    title: String,
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    widths: Vec<usize>,
}

pub struct TableBuilder {
    table: Table,
}

impl TableBuilder {
    pub fn new() -> Self {
        Self {
            table: Table {
                title: String::new(),
                headers: vec![],
                rows: vec![],
                widths: vec![],
            },
        }
    }

    pub fn set_title(mut self, title: &str) -> Self {
        self.table.title = title.to_owned();
        self
    }

    pub fn add_column(mut self, header: &str, width: usize) -> Self {
        self.table.headers.push(header.to_owned());
        self.table.widths.push(width);
        self
    }

    pub fn build(self) -> Table {
        self.table
    }
}

impl Table {
    pub fn builder() -> TableBuilder {
        TableBuilder::new()
    }

    pub fn add_row(&mut self, row: Vec<String>) -> Result<()> {
        if row.len() != self.headers.len() {
            return Err(anyhow!(format!(
                "Wrong number of columns in row, provided {}, expected {}",
                row.len(),
                self.headers.len()
            )));
        }
        self.rows
            .push(row.iter().map(|value| value.trim().to_owned()).collect());
        Ok(())
    }

    fn generate_title(&self) -> String {
        format!("{:-^1$}", self.title, self.widths.iter().sum::<usize>())
    }

    fn generate_header(&self) -> String {
        self.headers
            .iter()
            .zip(&self.widths)
            .map(|(header, width)| format!("{:^1$}", header, *width))
            .collect::<Vec<_>>()
            .join("|")
    }

    fn generate_row(&self, row: &Vec<String>) -> String {
        row.iter()
            .zip(&self.widths)
            .map(|(cell, width)| Self::format_cell_content(cell, *width))
            .collect::<Vec<_>>()
            .join("|")
    }

    fn generate_table(&self) -> String {
        let mut result = String::new();
        writeln!(&mut result, "{}", self.generate_title()).unwrap();
        writeln!(&mut result, "{}", self.generate_header()).unwrap();
        self.rows
            .iter()
            .for_each(|row| writeln!(&mut result, "{}", self.generate_row(row)).unwrap());
        result
    }

    fn format_cell_content(text: &str, width: usize) -> String {
        if text.len() <= width {
            return format!("{:<1$}", text, width);
        }
        if width <= 3 {
            return format!("{:.<1$}", "", width);
        }
        format!("{:<1$}", text.truncate_ellipse(width - 3), width)
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.generate_table())
    }
}
