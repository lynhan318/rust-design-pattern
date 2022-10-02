// Prototype Pattern
// Intent:
// Specify the kinds of objects to create using prototypical instance, and create new objects by
// copying this prototype
// ┌───────────────────────────┐
// │                           │
// │        Client             ├─────────────────────────────┐             ┌────────────────────┐
// ├───────────────────────────┤                             │             │                    │
// │                           │                             │             │     Prototype      │
// │     operation()           │                             │             └────────────────────┤
// │          │                │                             └─────────────►                    │
// │          │                │                                           │   Clone()          │
// └──────────┼────────────────┘                                           └─────────▲──────────┘
//            │                                                                      │
//            │                                                                      │
//            │                                                                      │
//            │                                                                      │
//            │                                                                      │
//            │                                           ┌──────────────────────────┴──────────────────────────┐
//     object=prototype.clone()                           │                                                     │
//                                                        │                                                     │
//                                                        │                                                     │
//                                                        │                                                     │
//                                                        │                                                     │
//                                              ┌─────────┴───────────────┐                       ┌─────────────┴──────────────┐
//                                              │                         │                       │                            │
//                                              │ ConcretePrototype1      │                       │  ConcretePrototype2        │
//                                              ├────────────────────────┬┤                       ├────────────────────────────┤
//                                              │                         ┤                       │                            │
//                                              │     Clone()             │                       │        Clone()             │
//                                              └─────────────────────────┘                       └────────────────────────────┘
// NOTED: when instances of a class can have one of only a few different combinations of state.
// It may be more convenient to install a corresponding number of prototypes and clone them
// rather than instantiating the class manually, each time with the appropriate state.
// Implementation

#[derive(Debug)]
struct Pen {
    pub color: String,
    pub width: i64,
}

trait PenPrototype {
    fn clone(&self) -> Pen;
    fn set_color(&self, color: String) -> Pen;
    fn set_width(&self, width: i64) -> Pen;
}

impl PenPrototype for Pen {
    fn clone(&self) -> Pen {
        Self {
            width: self.width.clone(),
            color: self.color.clone(),
        }
    }

    fn set_width(&self, width: i64) -> Pen {
        Pen {
            width,
            color: self.color.clone(),
        }
    }
    fn set_color(&self, color: String) -> Pen {
        Pen {
            width: self.width.clone(),
            color,
        }
    }
}

struct Circle {
    pub x: i64,
    pub y: i64,
    pub r: i64,
}

struct Rectangle {
    pub width: i64,
    pub height: i64,
}

trait Draw {
    fn draw(&self, pen: Pen);
}

impl Draw for Circle {
    fn draw(&self, pen: Pen) {
        iprintlnprintln!("Draw Circle with pen {:?}", pen);
    }
}

impl Draw for Rectangle {
    fn draw(&self, pen: Pen) {
        iprintlnprintln!("Draw Rectangle with pen {:?}", pen);
    }
}

fn demo_prototype() {
    let pen = Pen {
        width: 2,
        color: "green".into(),
    };
    //draw circle
    Circle { x: 1, y: 2, r: 3 }.draw(pen.set_color("green".into()));

    //draw rectangle
    Rectangle {
        width: 1,
        height: 2,
    }
    .draw(pen.set_width(3));
}
