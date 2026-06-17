// Main
use std::fmt::format;
use clap::{Parser, Subcommand};
use chrono::NaiveDate;

mod task;
mod task_list;

use task::Priority;
use task_list::TaskList;

fn parse_date(s: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(s, "%d/%m/%Y")
}

#[derive(Parser)]
#[command(name = "Taskr")]
#[command(version = "1.0")]
#[command(propagate_version = true)]
#[command(about = "Gestionnaire de tâches en CLI.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ajoute une tâche
    Add { 
        /// Titre de la tâche
        title: String,

        /// Date d'échéance [format: jj/mm/aaaa]
        #[arg(value_parser = parse_date)]
        due_date: NaiveDate,

        #[arg(short, long, value_enum)]
        priority: Option<Priority>,
    },
    
    /// Valide une tâche
    Done { id: u32 },

    /// Liste les tâches par ordre de priorité
    List,

    /// Supprime une tâche
    Remove { id: u32 },

    /// Supprime les tâches terminées
    ClearDone,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut list = TaskList::load()?;

    match &cli.command {
        Commands::Add { title, priority, due_date } => {
            let priority = match priority {
                Some(p) => p.clone(),
                None => Priority::Medium
            };
            let id = list.add(title, priority, *due_date);
            println!("Tâche # {id} ajoutée.");
        }
        Commands::Remove { id } => {
            if list.remove(*id) {
                println!("Tâche #{id} supprimée.");
            } else {
                println!("Pas de tâche #{id}.");
            }
        }
        Commands::Done { id } => {
            if list.complete(*id) {
                println!("Tâche #{id} terminée ✓");
            } else {
                println!("Pas de tâche #{id}.");
            }
        }
        Commands::ClearDone => {
            let deleted = list.remove_completed();
            println!("{deleted} tâches terminées ont été supprimées.")
        }
        Commands::List => list.list(),
    }
    list.save()?;
    Ok(())
}
