use structural::{adapter, bridge, composite, decorator, facade, proxy, proxy_nginx};

fn main() {
    // bridge::demo_bridge();
    // composite::create_desktop();
    // adapter::demo_adapter();
    // decorator::demo_decorator();
    // facade::demo_facade().unwrap();

    proxy_nginx::demo();

    println!("=====================================");
}
