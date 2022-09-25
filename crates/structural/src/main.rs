use structural::{adapter, bridge, composite, decorator, facade};

fn main() {
    bridge::demo_bridge();
    composite::create_desktop();
    adapter::demo_adapter();
    decorator::demo_decorator();
    facade::demo_facade().unwrap();

    println!("=====================================");
}
