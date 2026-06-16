// Main
mod task;
mod task_list;

use task::Priority;
use task_list::TaskList;

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
