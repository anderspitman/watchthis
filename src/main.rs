use notify::{RecursiveMode, Watcher, EventKind, event::{ModifyKind,DataChange}};
use std::sync::mpsc;
use std::path::Path;
use std::process::Command;

fn main() -> notify::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let dir_str = &args[1];
    let program = &args[2];

    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    let dir = std::path::absolute(Path::new(dir_str))?;

    watcher.watch(&dir, RecursiveMode::Recursive)?;

    let excluded_dirs = [".git", "node_modules"];

    for res in rx {
        match res {
            Ok(event) => {
                //println!("event: {:?}", event);
                if event.kind == EventKind::Modify(ModifyKind::Data(DataChange::Any)) {

                    let path = &event.paths[0];

                    if contains_excluded_dir(path, &excluded_dirs) {
                        continue;
                    }

                    let rel_path = path.strip_prefix(&dir).unwrap();

                    Command::new("sh")
                        .arg("-c")
                        .arg(vec![program.clone(), rel_path.to_str().unwrap().to_string()].join(" "))
                        .status()
                        .expect("failed to execute process");
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn contains_excluded_dir<P: AsRef<Path>>(path: P, excluded: &[&str]) -> bool {
    path.as_ref()
        .ancestors()
        .any(|ancestor| {
            ancestor
                .components()
                .any(|component| {
                    excluded.iter().any(|&ex| component.as_os_str() == ex)
                })
        })
}
