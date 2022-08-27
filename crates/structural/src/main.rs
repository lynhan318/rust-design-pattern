use structural::{adapter, bridge, composite, decorator};

fn main() {
    bridge::demo_bridge();
    composite::create_desktop();
    adapter::demo_adapter();
    decorator::demo_decorator();

    println!("=====================================");
}
