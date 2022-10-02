pub mod adapter;
pub mod bridge;
pub mod composite;
pub mod decorator;
pub mod facade;
pub mod flyweight;
pub mod proxy;
pub mod proxy_nginx;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
