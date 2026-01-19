use nucleus_std::neutron::store;
use serde::Serialize;

#[derive(Clone, Serialize, Debug, Default)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

#[store]
pub struct TodoStore {
    pub todos: Vec<Todo>,
    pub filter: String,
}

impl TodoStore {
    pub fn add_todo(&self, text: String) {
        self.todos.modify(|t| {
            t.push(Todo {
                id: fastrand::u64(..),
                text,
                completed: false,
            });
            println!("✅ Added todo. Count: {}", t.len());
        });
    }

    pub fn toggle(&self, id: u64) {
        self.todos.modify(|list| {
            if let Some(todo) = list.iter_mut().find(|t| t.id == id) {
                todo.completed = !todo.completed;
                println!("🔄 Toggled todo: {}", id);
            }
        });
    }

    pub fn set_filter(&self, f: String) {
        println!("🔍 Filter changed to: {}", f);
        self.filter.set(f);
    }

    // Derived State (Computed on demand)
    pub fn filtered_todos(&self) -> Vec<Todo> {
        let f = self.filter.get();
        let list = self.todos.get();

        match f.as_str() {
            "active" => list.into_iter().filter(|t| !t.completed).collect(),
            "completed" => list.into_iter().filter(|t| t.completed).collect(),
            _ => list,
        }
    }
}
