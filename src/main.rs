// Main
mod task;
mod task_list;

use std::fmt::format;

use task::Priority;
use task_list::TaskList;

use clap::{Parser, Subcommand};

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
        Commands::Add { title, priority } => {
            let priority = match priority {
                Some(p) => p.clone(),
                None => Priority::Medium
            };
            let id = list.add(title, priority);
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
