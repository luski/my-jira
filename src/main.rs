use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut navigator = Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();

        match navigator.get_current_page() {
            Some(current_page) => {
                if let Err(error) = current_page.draw_page() {
                    handle_error(error);
                    break;
                }
                let user_input = get_user_input();

                let action = match current_page.handle_input(&user_input) {
                    Ok(action) => action,
                    Err(error) => {
                        handle_error(error);
                        break;
                    }
                };

                if let Some(action) = action {
                    if let Err(error) = navigator.handle_action(action) {
                        handle_error(error);
                        break;
                    }
                }
            }
            None => break,
        }
    }
}

fn handle_error(error: anyhow::Error) {
    println!("Error rendering page: {}", error);
    println!("Press any key to continue...");
    wait_for_key_press();
}
