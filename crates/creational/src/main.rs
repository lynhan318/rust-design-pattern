use creational::{builder, factory};

fn demo_factory() {
    let client = factory::get_widget_factory(factory::Themes::Motif);
    let window = client.create_window();
    let scroll = client.create_scroll_bar();
    let info = client.get_theme_info();
    println!("Client::Window {:?}", window);
    println!("Client::ScrollBar {:?}", scroll);
    println!("Client::ThemeInfo {:?}", info);

    println!("----------------------");
    let client = factory::get_widget_factory(factory::Themes::PMWindow);
    let window = client.create_window();
    let scroll = client.create_scroll_bar();
    println!("Client::Window {:?}", window);
    println!("Client::ScrollBar {:?}", scroll);
    println!("Client::ThemeInfo {:?}", info);
}
fn demo_builder() {
    let query_builder = builder::QueryBuilder::new("user")
        .select("id,username,display_name")
        .limit(10)
        .offset(5)
        .order_by("created_at DESC")
        .build();
    println!("Your Query::{}", query_builder);

    let query_builder = builder::QueryBuilder::new("user")
        .select("id,username,display_name")
        .limit(0)
        .build();

    println!("Your Query::{}", query_builder);
}
fn main() {
    demo_builder();
}
