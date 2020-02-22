#[macro_use]
extern crate anyhow;

mod entity;
mod error;
mod value_object;

pub use error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
