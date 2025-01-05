use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
