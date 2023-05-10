use crate::strategy::{Bicyle, MotorBike};

mod command;
mod iter;
mod strategy;
mod mediator;

pub fn main() {
    let moving = strategy::Move::new("HCM", "Dallas");
    let path_found = moving.find_path(Bicyle);
    dbg!(&path_found);
    let path_found = moving.find_path(MotorBike);
    dbg!(&path_found);
    print!("Run");
}
