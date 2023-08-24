use ellipse::Ellipse;
use anyhow::{anyhow, Result};
use itertools::Itertools;

pub fn get_column_string(text: &str, width: usize) -> String {
    if text.len() <= width {
        return format!("{:<1$}", text, width);
    }
    if width <= 3 {
        return format!("{:.<1$}", "", width);
    }
    format!("{:<1$}", text.truncate_ellipse(width - 3), width)
}


pub fn draw_table(
    title: &str,
    headers: &Vec<String>,
    rows: &Vec<Vec<String>>,
    widths: &Vec<usize>,
) -> Result<()> {
    let cols = headers.len();

    if widths.len() != cols {
        return Err(anyhow!("Wrong number of widths"));
    }

    if rows.iter().any(|row| row.len() != cols) {
        return Err(anyhow!("Wrong number of columns in rows"));
    }
    println!("{:-^1$}", title, widths.iter().sum::<usize>());

    let print_row = |values: &Vec<String>| {
        println!(
            "{}",
            values
                .iter()
                .zip(widths)
                .map(|(header, width)| format!("{:^1$}", get_column_string(header, *width), *width))
                .join("|")
        );
    };

    print_row(headers);

    for row in rows {
        print_row(row);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    } 
}