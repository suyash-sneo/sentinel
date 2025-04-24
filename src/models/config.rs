use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Trigger {
    paths: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Action {
    what: String,
    workdir: String,
    exec: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    trigger: Trigger,
    action: Action,
}
