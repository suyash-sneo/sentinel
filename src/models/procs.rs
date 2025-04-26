use std::sync::{Arc, Mutex};
use std::process::Child;

pub struct ChildProc {
    pub name: String,
    pub pid: u32,
    pub arc: Arc<Mutex<Option<Child>>>,
    pub signal_arc: Arc<Mutex<Option<Child>>>,
}
