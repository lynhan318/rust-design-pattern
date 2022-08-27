// Intent:
// compose objects into tree structures to represent part-whole hierarchies.
// Composite lets clients treat individual objects and compositions of objects uniformly.
//
// ┌────────────────┐
// │                │        ┌──────────────────┐
// │  Client        ├────────►    Component     │◄─────────────────────┐
// │                │        ┌──────────────────┤                      │
// └────────────────┘        │                  │                      │
//                           │ Operation()      │                      │
//                           └──────┬───────────                       │
//                                  │                                  │
//                                  │                     agregator <- │
//                                  │                                  │
//                                  │immplementation                   │
//                                  │                                  │
//           ┌───────────────┬──────┴──┬───────────────────┐  childrens│
//           │   Leaf        │         │      Composite    ├───────────┘
//           ├───────────────┤         ├───────────────────┤
//           │               │         │Operation()        │
//           │ Operation()   │         │Add(Component)     │
//           │               │         │Remove(Component)  │
//           └───────────────┘          +childrens:Component[]
//                                     └───────────────────┘
pub trait Graphic {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Circle {
    pub radius: i64,
}

impl Graphic for Circle {
    fn draw(&self) {
        println!("Draw Circle({})", self.radius);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub x: i64,
    pub y: i64,
}
impl Graphic for Rectangle {
    fn draw(&self) {
        println!("draw Rectangle({},{})", self.x, self.y);
    }
}

#[derive(Debug)]
pub struct Line {
    pub x: i64,
    pub y: i64,
}
impl Graphic for Line {
    fn draw(&self) {
        println!("Draw Line({},{})", self.x, self.y);
    }
}

pub struct Desktop {
    pub graphics: Vec<Box<dyn Graphic>>,
}

impl Graphic for Desktop {
    fn draw(&self) {
        for g in self.graphics.iter() {
            g.draw();
        }
    }
}
impl Desktop {
    fn new() -> Desktop {
        Desktop { graphics: vec![] }
    }
    pub fn remove(mut self, idx: i64) -> bool {
        self.graphics.remove(idx as usize);
        true
    }
    pub fn get(&self, idx: usize) -> Option<&Box<dyn Graphic>> {
        self.graphics.get(idx)
    }

    pub fn add(&mut self, graphic: Box<dyn Graphic>) -> i64 {
        self.graphics.push(graphic);
        self.graphics.len() as i64 - 1
    }
}

pub fn create_desktop() {
    let mut desk = Desktop::new();
    // create line
    let line_idx = desk.add(Box::new(Line { x: 10, y: 20 }));
    let line = desk.get(line_idx as usize);
    println!("Rectangle idx::{} >>>>>>", line_idx);
    line.unwrap().draw();
    println!("Rectangle idx::{} <<<<<", line_idx);

    // create circle
    let cir_idx = desk.add(Box::new(Circle { radius: 4 }));
    let cir = desk.get(cir_idx as usize);
    println!("Circle idx::{} >>>>>>", cir_idx);
    cir.unwrap().draw();
    println!("Circle idx::{} <<<<<", cir_idx);

    // create Rectangle
    let rec_idx = desk.add(Box::new(Rectangle { x: 12, y: 24 }));
    let rec = desk.get(cir_idx as usize);
    println!("Rectangle idx::{} >>>>>>", rec_idx);
    rec.unwrap().draw();
    println!("Rectangle idx::{} <<<<<", rec_idx);

    // draw
    desk.draw();
}
