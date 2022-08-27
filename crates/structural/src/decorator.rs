// INTENT
// (Decorate as known as Wrapper)
// Attach additional responsibilities to an object dynamically. Decorators provide a
// flexible alternative to subclassing for extending functionality.
// Motivate
// A more FLEXIBLE APPROACH IS TO ENCLOSE THE COMPONENT IN ANOTHER OBJECT THAT ADDS
// THE BORDER. tHE ENCLOSING OBJECT IS CALLED A DECORATOR. The decorator conforms to
// the interface ofthe component it decoratesso that its presence istransparent to the
// component's clients. The decorator forwards requests to the component and may
// perform additional actions (such as drawing a border) before or after forwarding.
// Transparency lets you nest decorators recursively, thereby allowing an unlimited
// number of added responsibilities.

enum Size {
    S,
    M,
    L,
}

struct Coffee {
    pub size: Size,
}

struct MilkCoffee {
    pub coffee: Coffee,
    pub size: Size,
}

trait Order {
    fn make(&self);
}
trait DecorateOrder: Order {
    fn add_milk(&self);
}

impl Order for Coffee {
    fn make(&self) {
        println!("Making coffee")
    }
}
impl Order for MilkCoffee {
    fn make(&self) {
        self.coffee.make();
    }
}

impl DecorateOrder for MilkCoffee {
    fn add_milk(&self) {
        self.make();
        println!("Adding milk to coffee")
    }
}

pub fn demo_decorator() {
    let coffee = Coffee { size: Size::S };

    coffee.make();

    println!("===================");
    let milk_coffee = MilkCoffee {
        coffee,
        size: Size::M,
    };

    milk_coffee.add_milk();
}
