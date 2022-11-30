use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use crate::srack_value::SrackValue;

pub struct Srack {
    collection: Vec<SrackValue>,
    random: Option<StdRng>,
}

impl Srack {
    pub fn new() -> Srack {
        Srack {
            collection: Vec::new(),
            random: None,
        }
    }

    pub fn init(&mut self, seed: u64) {
        self.random = Some(StdRng::seed_from_u64(seed))
    }

    pub fn push(&mut self, item: SrackValue) {
        match &mut self.random {
            None => { panic!("Srack didn't initialized") }
            Some(r) => {
                let i = &r.gen_range(1..4);
                match i {
                    1 => {
                        println!(">log: push");
                        self.collection.push(item);
                    }
                    2 => { println!(">log: do nothing"); }
                    3 => {
                        println!(">log: push x2");
                        self.collection.push(item.clone());
                        self.collection.push(item);
                    }
                    4 => {
                        println!(">log: pop & push");
                        self.collection.pop();
                        self.collection.push(item);
                    }
                    &_ => { panic!("Invalid random value {i}") }
                }
            }
        }
    }

    pub fn poop(&mut self) {
        match &mut self.random {
            None => { panic!("Srack didn't initialized") }
            Some(r) => {
                let count = *&r.gen_range(1..10);
                for _ in 0..count {
                    println!(">log: Poop item");
                    self.collection.pop();
                }
            }
        }
    }

    pub fn peek(&self, offset: usize) -> Option<&SrackValue> {
        if self.collection.len() > offset {
            let idx = self.collection.len() - offset - 1;
            Some(&self.collection[idx])
        } else { None }
    }
}