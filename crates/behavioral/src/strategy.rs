//STRATEGY:
//Strategy is a behavioral design pattern that lets you define a family of algorithms,
//put each of them into a separate class, and make their objects interchangeable.
#[derive(Debug)]
pub struct PathResult {
    pub hour: u8,
    pub kilometers: i64,
}
pub trait Route {
    fn build_route(&self, from: &str, to: &str) -> PathResult;
}

pub struct Bicyle;
impl Route for Bicyle {
    fn build_route(&self, from: &str, to: &str) -> PathResult {
        println!("moving from {} to {} by Bicyle take 8hours", from, to);
        PathResult {
            hour: 8,
            kilometers: 300,
        }
    }
}

pub struct MotorBike;
impl Route for MotorBike {
    fn build_route(&self, from: &str, to: &str) -> PathResult {
        println!("moving from {} to {} by MotorBike take 2hours", from, to);
        PathResult {
            hour: 2,
            kilometers: 500,
        }
    }
}

pub struct Move<'a> {
    pub from: &'a str,
    pub to: &'a str,
}

impl<'a> Move<'a> {
    pub fn new(from: &'a str, to: &'a str) -> Self {
        Self { from, to }
    }
    pub fn find_path(&self, strategy: impl Route) -> PathResult {
        strategy.build_route(&self.from, &self.to)
    }
}
