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

    for res in rx {
        match res {
            Ok(event) => {
                //println!("event: {:?}", event);
                if event.kind == EventKind::Modify(ModifyKind::Data(DataChange::Any)) {

                    let path = &event.paths[0];
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
