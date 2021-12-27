use crate::types::Commands::{Add, Complete, Edit, Incomplete, List, Remove};
use crate::{cli, controllers, db};

pub fn app() {
    let conn = db::init_db();
    let command = cli::get_command();

    let res = match command {
        Add(text) => controllers::add_todo(&conn, &text),
        Incomplete(id) => controllers::mark_todo_incomplete(&conn, id),
        Complete(id) => controllers::mark_todo_complete(&conn, id),
        Edit(id, text) => controllers::edit_todo(&conn, id, &text),
        Remove(id) => controllers::remove_todo(&conn, id),
        List => {
            let todos = controllers::list(&conn);
            if let Ok(todos) = todos {
                for todo in todos {
                    println!("{:?}", todo);
                }
            }
            Ok(0)
        }
    };

    if let Err(e) = res {
        println!("Error Encountered: {}", e);
    }
}
