use surrealdb::{engine::remote::ws::Client, Surreal};

pub mod endpoint;
pub mod jwt;
pub mod config;

pub type SurrealDb = Surreal<Client>;