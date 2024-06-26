pub mod edge;
pub mod model;
pub mod nodes;
pub mod parameters;
pub mod tables;

pub use model::{PywrModel, PywrNetwork};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
