#![warn(clippy::all, clippy::pedantic)]

mod app;
mod cli;
mod controllers;
mod db;
mod types;

fn main() {
    app::app();
}
