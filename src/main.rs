#[macro_use]
extern crate rocket;
use serde_json::json;
use serde::{Deserialize,Serialize};
use rocket::serde::json::Json;

// #[get("/")]
// fn index() -> &'static str {
    // "Hello, world!"
// }

struct Todo {
    todos: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}
	

impl Todo {
    fn new(input_todos: Vec<String>) -> Self {
        Self { todos: input_todos }
    }
    // pub

    fn get_todos(&self) -> String {
        let mut result = String::from("");
        for todo in &self.todos {
            result.push_str(&format!("{}\n",todo));
        }
        result
	    // .to_string()
        // "Todo: list of todos"
    }
}

#[get("/")]
fn todos() -> String {
    let my_todos = vec![
        "emacs".to_string(),
        "xmonad".to_string(),
        "communist".to_string(),
    ];

    let todo_object = Todo::new(my_todos);
    let result = todo_object.get_todos().to_string();
    // &result
    let mut returned = "kalvakuntla";
    result
}

#[get("/json")]
fn my_json_todo() -> Json<Person> {
    let person = Person {
	name: "Farm house".to_string(),
	age: 10,
    };
    
    // let j = serde_json::to_string(&person);

    Json(person)

}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![my_json_todo])
    // todo_object.todos()])
}
