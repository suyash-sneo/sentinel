use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Trigger {
    pub paths: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Action {
    pub what: String,
    pub workdir: String,
    pub exec: String,
    pub args: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Output {
    pub mode: String,
    pub path: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    pub trigger: Trigger,
    pub action: Action,
    pub output: Output,
}
