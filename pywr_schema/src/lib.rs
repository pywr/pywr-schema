mod edge;
mod model;
pub mod nodes;
pub mod parameters;

pub use model::PywrModel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
