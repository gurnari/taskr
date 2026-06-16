// TaskList Module
use serde::{Serialize, Deserialize};
use std::fs;

use crate::task::{Task, Priority};

const FILE: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
pub struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: Vec::new(), next_id: 1 }
    }

    pub fn add(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.tasks.push(Task::new(id, title, priority));
        self.next_id += 1;
        id
    }

    fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    pub fn complete(&mut self, id: u32) -> bool {
        match self.find_mut(id) {
            Some(task) => { task.complete(); true }
            None => false,
        }
    }

    pub fn remove(&mut self, id: u32) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < before
    }

    pub fn list(&self) {
        if self.tasks.is_empty() {
            println!("Aucune tâche. Ajoutez-en une avec `add` !");
            return;
        }
        for task in &self.tasks {
            println!("{task}");
        }
    }

    pub fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.is_done()).count()
    }

    pub fn high_priority(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.priority == Priority::High).collect()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(FILE, json)?;
        Ok(())
    }

    pub fn load() -> Result<TaskList, Box<dyn std::error::Error>> {
        match fs::read_to_string(FILE) {
            Ok(json) => Ok(serde_json::from_str(&json)?),
            Err(_) => Ok(TaskList::new()),
        }
    }
}

