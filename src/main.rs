use notify::{RecursiveMode, Watcher, EventKind, event::{ModifyKind,DataChange}};
use std::sync::mpsc;
use std::path::Path;
use std::process::Command;

fn main() -> notify::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let dir = &args[1];
    let program = &args[2];

    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new(dir), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in
    for res in rx {
        match res {
            Ok(event) => {
                //println!("event: {:?}", event);
                if event.kind == EventKind::Modify(ModifyKind::Data(DataChange::Any)) {
                    //println!("event: {:?}", event.paths[0]);

                    let path_str = event.paths[0].clone().into_os_string().into_string().unwrap();
                    let parts = path_str.split(dir).collect::<Vec<&str>>();
                    //let path = "./".to_string() + parts[1];
                    let path = parts[1].to_string();

                    //Command::new(&program)
                    //    .args(&args[3..])
                    Command::new("sh")
                        .arg("-c")
                        .arg(vec![program.clone(), path].join(" "))
                        //.args(&args[2..])
                        .status()
                        .expect("failed to execute process");

                    //println!("{:?}", status)
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
