#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate diesel;

mod c2_value_object;
mod c3_entity;
mod c4_domain_service;
mod c5_repository;
mod error;

pub use error::*;
