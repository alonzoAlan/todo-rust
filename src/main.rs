#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rocket::serde::json::Value;

use serde::{Deserialize, Serialize};
use serde_json::json;
// use serde_json::json::Value;

// #[get("/")]
// fn index() -> &'static str {
// "Hello, world!"
// }

struct Todos {
    todos: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct Todo {
    task: String,
    // age: u8,
}

// impl<'r> Responder<'r> for Vec<Json<Person>> {
// fn respond_to(self, _: &Request<'_>) -> rocket::response::Result<'r> {
// for element in self {
// Response::build().sized_body()
// }
// }
// }

// impl Todo {

// }

impl Todos {
    fn new(input_todos: Vec<String>) -> Self {
        Self { todos: input_todos }
    }

    fn get_todos(&self) -> String {
        let mut result = String::from("");
        for todo in &self.todos {
            result.push_str(&format!("{}\n", todo));
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

    let todo_object = Todos::new(my_todos);
    let result = todo_object.get_todos().to_string();
    // &result
    let mut returned = "kalvakuntla";
    result
}

// #[get("/json")]
// fn my_json_person() -> Vec<Json<Person>> {
// let person = Person {
// name: "Farm house".to_string(),
// age: 10,
// };
// vec![Json(person)]
// }

#[get("/todos")]
fn my_json_todo() -> Json<Todo> {
    let todo = Todo {
        task: "Emacs".to_string(),
        // age: 10,
    };

    Json(todo)
}

#[get("/multi")]
fn multi_json_todo() -> Value {
    let todo = Todo {
        task: "Emacs".to_string(),
        // age: 10,
    };

    let todo2 = Todo {
        task: "RGV".to_string(),
        // age: 10,
    };

    json!([*Json(todo),*Json(todo2)])
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![multi_json_todo])
    // todo_object.todos()])
}
