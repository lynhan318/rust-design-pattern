// ITERATOR:
// is a behavioral design pattern that lets you traverse elements of a collection
// without exposing its underlying representation (list, stack, tree, etc.).
//
//     Iterator                IteratorCollection
//     ┌─────────────┐        ┌──────────────┐
//     │             │        │              │
//     │             │        │              │
//     │+next()      │        │ + iter()->Iterator
//     │+has_more()  │        │              │
//     └──────▲──────┘        └──────────────┘
//            │
//            │
//            │
//            │
//            │
// ConcreteIterator null
//    ┌───────────────────┐          ConcreateCollection
//    │-index             │         ┌─────────────────┐
//    │                   └◄───────►│                 │
//    │-collection:ConcretCollection│                 │
//    ├───────────────────┐         │                 │
//    │                   │         │                 │
//    │+next()            │         │+ iter()->Iterator
//    │+has_more()        │         │                 │
//    └───────────────────┘         └─────────────────┘
//
//
pub struct AnimalCollection {
    pub animal: [&'static str; 3],
}

impl AnimalCollection {
    fn iter(&self) -> AnimalIterator {
        AnimalIterator {
            index: 0,
            collections: self,
        }
    }
}

pub struct AnimalIterator<'a> {
    index: usize,
    collections: &'a AnimalCollection,
}

impl Iterator for AnimalIterator<'_> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.collections.animal.len() {
            let value = self.collections.animal[self.index];
            self.index += 1;
            return Some(value);
        }
        None
    }
}
