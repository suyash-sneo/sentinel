use notify::{PollWatcher, RecursiveMode, Watcher, Config, Event};
use std::time::Duration;
use std::path::Path;
use std::fs::File;
use std::process::{Command, Stdio, Child};
use std::io::{BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
use std::os::unix::process::CommandExt;
use signal_hook::{consts::signal::*, iterator::Signals};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

mod models;

fn listen_change() -> notify::Result<()> {

    let callback = |res: notify::Result<Event>| {
        match res {
            Ok(event) => println!("Changed: {:?}", event),
            Err(err) => println!("Watch error: {:?}", err),
        }
    };

    let mut watcher = PollWatcher::new(
        callback,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )?;

    watcher.watch(Path::new("/Users/sneo/dev/code/backend/sentinel/sample"), RecursiveMode::Recursive);

    loop{
        std::thread::sleep(Duration::from_secs(120));
    }
}

fn parse_config(file_path: &str) -> models::config::Config {

    let config_file = File::open(file_path).expect("Error opening config file");
    let reader = BufReader::new(config_file);

    let config: models::config::Config = serde_yaml::from_reader(reader).expect("Error parsing yaml");

    return config;
}

fn launch_command(config: models::config::Config) {

    let arg1 = &config.action.args[0];
    let arg2 = &config.action.args[1];

    let mut child = Command::new(config.action.exec)
        .arg(arg1)
        .arg(arg2)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start child process");

    let child_pid = child.id();

    let child_arc = Arc::new(Mutex::new(Some(child)));

    // Setup signal handling
    let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
    let signal_child = Arc::clone(&child_arc);

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal: {:?}", sig);
            if let Some(ref mut child) = *signal_child.lock().unwrap() {
                let _ = kill(Pid::from_raw(child_pid as i32), Signal::try_from(sig).unwrap());
            }
        }
    });

    if let Some(stdout) = child_arc.lock().unwrap().as_mut().unwrap().stdout.take() {
        let bufreader = BufReader::new(stdout);
        for line in bufreader.lines() {
            println!("Stdout: {}", line.unwrap());
        }
    }

    // Wait for the child to exit
    if let Some(mut child) = child_arc.lock().unwrap().take() {
        let _ = child.wait();
    }
}

fn main() {

    let config = parse_config("/Users/sneo/dev/code/backend/sentinel/sample-configs/go-server.yaml");

    println!("Parsed config: {:#?}", config);

    launch_command(config);
}
