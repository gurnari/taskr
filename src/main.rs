use std::fmt::{self, format};
use serde::{Serialize, Deserialize};
use std::fs;

const FILE: &str = "tasks.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Status {
    Todo,
    Done,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    fn symbol(&self) -> &str {
        match self {
            Priority::Low => ".",
            Priority::Medium => "!",
            Priority::High => "!!",
        }
    }

    fn from_str(s: &str) -> Priority {
        match s {
            "low" => Priority::Low,
            "high" => Priority::High,
            _ => Priority::Medium,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    title: String,
    status: Status,
    priority: Priority,
}

impl Task {
    // Fonction associée (pas de self) : c'est le "constructeur" idiomatique
    fn new(id: u32, title: &str, priority: Priority) -> Task {
        Task {
            id,
            title: title.to_string(),
            status: Status::Todo,
            priority,
        }
    }

    // &mut self : la méthode modifie la tâche
    fn complete(&mut self) {
        self.status = Status::Done;
    }

    // &self : lecture seule
    fn is_done(&self) -> bool {
        self.status == Status::Done
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mark = if self.is_done() { "x" } else { " " };
        write!(f, "[{}] #{} {} {}", mark, self.id, self.priority.symbol(), self.title)
    }
}

#[derive(Serialize, Deserialize)]
struct TaskList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskList {
    fn new() -> TaskList {
        TaskList { tasks: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, title: &str, priority: Priority) -> u32 {
        let id = self.next_id;
        self.tasks.push(Task::new(id, title, priority));
        self.next_id += 1;
        id
    }

    fn find_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|t| t.id == id)
    }

    fn complete(&mut self, id: u32) -> bool {
        match self.find_mut(id) {
            Some(task) => { task.complete(); true }
            None => false,
        }
    }

    fn remove(&mut self, id: u32) -> bool {
        let before = self.tasks.len();
        self.tasks.retain(|t| t.id != id);
        self.tasks.len() < before
    }

    fn list(&self) {
        if self.tasks.is_empty() {
            println!("Aucune tâche. Ajoutez-en une avec `add` !");
            return;
        }
        for task in &self.tasks {
            println!("{task}");
        }
    }

    fn pending_count(&self) -> usize {
        self.tasks.iter().filter(|t| !t.is_done()).count()
    }

    fn high_priority(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.priority == Priority::High).collect()
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(FILE, json)?;
        Ok(())
    }

    fn load() -> Result<TaskList, Box<dyn std::error::Error>> {
        match fs::read_to_string(FILE) {
            Ok(json) => Ok(serde_json::from_str(&json)?),
            Err(_) => Ok(TaskList::new()),
        }
    }
}

fn parse_id(s: &str) -> Result<u32, String> {
    let n = s.parse::<u32>().map_err(|_| format!("'{s}' n'est pas un id valide"))?;
    Ok(n)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut list = TaskList::load()?;

    match args.as_slice() {
        [cmd, title, rest @ ..] if cmd == "add" => {
            let priority = match rest.first().map(|s| s.as_str()) {
                Some("--priority") | Some("-p") => {
                    Priority::from_str(rest.get(1).map(|s| s.as_str()).unwrap_or("medium"))
                }
                _ => Priority::Medium,
            };
            let id = list.add(title, priority);
            println!("Tâche # {id} ajoutée.");
        }
        [cmd] if cmd == "list" => list.list(),
        [cmd, id] if cmd == "done" => {
            let id = parse_id(id)?;
            if list.complete(id) { println!("Tâche #{id} terminée ✓"); }
            else { println!("Pas de tâche #{id}."); }
        }
        [cmd, id] if cmd == "remove" => {
            let id = parse_id(id)?;
            if list.remove(id) { println!("Tâche #{id} supprimée."); }
            else { println!("Pas de tâche #{id}."); }
        }
        _ => {
            println!("Usage : taskr add <titre> [--priority low|medium|high]");
            println!("        taskr list | done <id> | remove <id>");
        }
    }

    list.save()?;
    Ok(())

}
