//MEDIATOR:
//is a behavioral design pattern that reduces coupling
//between components of a program
//by making them communicate indirectly, through a special mediator object.
//
//
//                            ┌───────────────────────┐
//                            │                       │
//                            │  Component            │
//                            │                       │
//           ┌───────────────►├───────────────────────┤ ◄─────────────────┐
//           │                │                       │                   │
//           │                │+businessLogic()       │                   │
//           │                │                       │                   │
//           │                └───────────────────────┘                   │
//           │                                                            │
// ┌─────────┴────────┐                                        ┌──────────┴──────┐
// │ ComponentA       │         ┌─────────────────────┐        │                 │
// │                  │         │ Mediator            │        │ ComponentB      │
// ├──────────────────┤         │                     │        │                 │
// │                  │         │                     │        ├─────────────────┘
// │- mediator:Mediator         ├─────────────────────┤        │-mediator:Mediator
// │                  │         │                     │        │                 │
// │                  │         │+ notify()           │        │                 │
// │+businessLogic()  │         │                     │        │+businessLogic() │
// └──────▲───────────┘         │                     │        └──────────────▲──┘
//        │                     └─────────▲───────────┘                       │
//        │                               │                                   │
//        │                               │                                   │
//        │                               │                                   │
//        │                               │                                   │
//        │                               │                                   │
//        │                               │                                   │
//        │                               │                                   │
//        │                               │                                   │
//        │                     ┌─────────┴────────────┐                      │
//        │                     │ConcreateMediator     │                      │
//        │                     │                      │                      │
//        │                     │                      │                      │
//        │                     ├──────────────────────┘                      │
//        │                     │- components:[]Components                    │
//        │                     │                      │                      │
//        └────────────────────[]                      │[]────────────────────┘
//                              │+ notify()            │
//                              │                      │
//                              │                      │
//                              └──────────────────────┘
use std::collections::{HashMap, VecDeque};

pub trait Train {
    fn name(&self) -> &String;
    fn arrive(&mut self, mediator: &mut dyn Mediator);
    fn depart(&mut self, mediator: &mut dyn Mediator);
}

pub trait Mediator {
    fn notify_about_arrival(&mut self, train_name: &str) -> bool;
    fn notify_about_departure(&mut self, train_name: &str);
}

struct HCMTrain {
    name: String,
}

impl Train for HCMTrain {
    fn name(&self) -> &String {
        &self.name
    }
    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&mut self.name) {
            println!("HCM train {} Arrival blocked, waiting", self.name);
            return;
        }
        println!("HCM train {}: Arrived", self.name);
    }
    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("HCM train {}: Leaving", self.name);
        mediator.notify_about_departure(&self.name)
    }
}

struct SETrain {
    name: String,
}
impl Train for SETrain {
    fn name(&self) -> &String {
        &self.name
    }
    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!("SE train {} Arrival blocked, waiting", self.name);
            return;
        }
        println!("SE train {}: Arrived", self.name);
    }
    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("SE train {}: Leaving", self.name);
        mediator.notify_about_departure(&self.name)
    }
}

#[derive(Default)]
pub struct TrainStation {
    trains: HashMap<String, Box<dyn Train>>,
    train_queue: VecDeque<String>,
    train_on_platform: Option<String>,
}

impl Mediator for TrainStation {
    fn notify_about_departure(&mut self, name: &str) {
        if Some(name.into()) == self.train_on_platform {
            println!("Train {} is ready for departure", name);
            self.train_on_platform = None;
            if let Some(next_train_name) = self.train_queue.pop_front() {
                let mut next_train = self.trains.remove(&next_train_name).unwrap();
                next_train.arrive(self);
                self.trains.insert(next_train_name.clone(), next_train);
                self.train_on_platform = Some(next_train_name);
            }
        }

        self.train_on_platform
            .replace(self.train_queue.pop_front().unwrap());

        println!("Train {} has departure", name);
    }
    fn notify_about_arrival(&mut self, train_name: &str) -> bool {
        if self.train_on_platform.is_some() {
            self.train_queue.push_back(train_name.into());
            false
        } else {
            self.train_on_platform.replace(train_name.into());
            true
        }
    }
}
impl TrainStation {
    fn accept(&mut self, mut train: impl Train + 'static) {
        if self.trains.contains_key(train.name()) {
            println!("{} already arrived", train.name());
            return;
        }
        train.arrive(self);
        self.trains.insert(train.name().clone(), Box::new(train));
    }
    fn depart(&mut self, name: &'static str) {
        let train = self.trains.remove(name);
        if let Some(mut train) = train {
            train.depart(self);
        } else {
            println!("{name} is not on the station");
        }
    }
}
