use notify::{PollWatcher, RecursiveMode, Watcher, Config, Event};
use std::time::Duration;
use std::path::Path;

fn main() -> notify::Result<()> {

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
