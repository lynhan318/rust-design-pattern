use structural::{adapter, bridge, composite};

fn main() {
    bridge::demo_bridge();

    println!("=====================================");

    composite::create_desktop();
}
