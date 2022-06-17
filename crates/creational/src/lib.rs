pub mod builder;
pub mod factory;
mod prototype;

fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
