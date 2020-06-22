#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

pub mod connection;
pub mod paginate;
pub use connection::*;
pub use paginate::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
