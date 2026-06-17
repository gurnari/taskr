// Task Module
use serde::{Deserialize, Serialize};
use std::fmt;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Status {
    Todo,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, clap::ValueEnum)]
pub enum Priority {
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

    pub fn from_str(s: &str) -> Priority {
        match s {
            "low" => Priority::Low,
            "high" => Priority::High,
            _ => Priority::Medium,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u32,
    title: String,
    status: Status,
    pub priority: Priority,
    due_date: NaiveDate,
}

impl Task {
    // Fonction associée (pas de self) : c'est le "constructeur" idiomatique
    pub fn new(id: u32, title: &str, priority: Priority, due_date: NaiveDate) -> Task {
        Task {
            id,
            title: title.to_string(),
            status: Status::Todo,
            priority,
            due_date,
        }
    }

    // &mut self : la méthode modifie la tâche
    pub fn complete(&mut self) {
        self.status = Status::Done;
    }

    // &self : lecture seule
    pub fn is_done(&self) -> bool {
        self.status == Status::Done
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mark = if self.is_done() { "x" } else { " " };
        write!(
            f,
            "[{}] #{} ({}) : {} | Échéance : {}",
            mark,
            self.id,
            self.priority.symbol(),
            self.title,
            self.due_date.format("%d/%m/%Y")
        )
    }
}
