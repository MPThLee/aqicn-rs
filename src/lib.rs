#[macro_use] extern crate serde_derive;

extern crate reqwest;
extern crate serde_json;
extern crate serde;

pub mod feed;
pub mod query;

mod get;

#[cfg(test)]
mod tests;
