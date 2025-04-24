use notify::{PollWatcher, RecursiveMode, Watcher, Config, Event};
use std::io::BufReader;
use std::time::Duration;
use std::path::Path;
use std::fs::File;

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

fn main() {

    let config_file = File::open("/Users/sneo/dev/code/backend/sentinel/sample-configs/go-server.yaml").expect("Error opening config file");
    let reader = BufReader::new(config_file);

    let config: models::config::Config = serde_yaml::from_reader(reader).expect("Error parsing yaml");

    println!("Parsed config: {:#?}", config);
}
