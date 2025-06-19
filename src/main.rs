// NOTE 1: A new dependency called clearscreen has been added. At the top of the program loop we call clearscreen::clear().
// This will clear the screen which is what we want to do before rendering the new content.
// Think about it like refreshing a web page... everything is wiped away and reloaded.

// NOTE 2: A function called wait_for_key_press() has been added to io_utils.rs. Use this method when displaying errors.
// For example:
// if let Err(error) = page.draw_page() {
//     println!("Error rendering page: {}\nPress any key to continue...", error);
//     wait_for_key_press();
// };

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
    let file_path = "data/mock.json";

    let db = Rc::new(JiraDatabase::new(file_path.to_string()));

    let mut nav = Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        let current_page = nav.get_current_page().unwrap();

        // 2. render page
        if let Err(error) = current_page.draw_page() {
            println!("Error: {}", error);
            wait_for_key_press();
        }

        // 3. get user input
        let input = get_user_input();
        println!("input: {}", input);

        // 4. pass input to page's input handler
        let action = current_page.handle_input(&input.trim()).unwrap();
        println!("action: {:?}", action);

        // sleep for 5 seconds
        std::thread::sleep(std::time::Duration::from_secs(5));

        // 5. if the page's input handler returns an action let the navigator process the action
        if let Some(action) = action {
            if let Err(error) = nav.handle_action(action) {
                println!("Error: {}", error);
                wait_for_key_press();
            }
        }
    }
}
