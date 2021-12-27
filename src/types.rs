use std::fmt::Debug;

#[derive(Debug)]
pub struct Todo {
    pub id: u64,
    pub completed: bool,
    pub text: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub enum Commands {
    Add(String),
    Edit(u64, String),
    Complete(u64),
    Incomplete(u64),
    List,
    Remove(u64),
}
