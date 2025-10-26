use notify::{RecursiveMode, Watcher};
use std::error::Error;
use std::path::Path;
use std::process::{Child, Command};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    args.next();
    let executable_path = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Must provide a path to an executable");
            std::process::exit(1);
        },
    };
    let should_restart_lock = start_runner(executable_path.clone());

    watch(executable_path, should_restart_lock)
}

fn watch(path: String, has_changed: Arc<Mutex<bool>>) -> Result<(), Box<dyn Error>> {
    // Create a channel to receive filesystem events
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(tx)?;
    // Watch the specific file (non-recursive, so only the file is monitored)
    let path_to_watch = Path::new(path.as_str());
    let file_name = path_to_watch.file_name().unwrap();
    watcher.watch(path_to_watch.parent().unwrap(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                for path in event.paths.as_slice() {
                    if path.ends_with(file_name) {
                        match event.kind {
                            notify::EventKind::Create(_) |
                            notify::EventKind::Modify(_) => {
                                *has_changed.lock().unwrap() = true;
                            },
                            _ => {}
                        }
                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e)
        }
    };
    Ok(())
}

fn start_runner(file_path: String) -> Arc<Mutex<bool>> {
    println!("Running executable: {}", file_path);
    let should_restart_lock = Arc::new(Mutex::new(false));
    let returnable_lock = Arc::clone(&should_restart_lock);
    thread::spawn(move || {
        let path = Path::new(file_path.as_str());
        let mut child_process = run_executable(path);
        loop {
            thread::sleep(Duration::from_secs(1));
            let mut should_restart = should_restart_lock.lock().unwrap();
            if *should_restart {
                println!("Reloading program");
                child_process.kill().unwrap();
                child_process = run_executable(Path::new(file_path.as_str()));
                *should_restart = false
            };
        }
    });
    returnable_lock
}

fn run_executable(path: &Path) -> Child {
    Command::new(path)
        .spawn()
        .expect("Failed to run program")
}

