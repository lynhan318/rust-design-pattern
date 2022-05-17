use creational::factory;

fn main() {
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
