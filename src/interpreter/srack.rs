use std::io::Write;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use super::srack_value::SrackValue;

pub(crate) struct Srack {
    collection: Vec<SrackValue>,
    random: Option<StdRng>,
    verbose: bool,
}

impl Srack {
    pub fn new(verbose: bool) -> Srack {
        Srack {
            collection: Vec::new(),
            random: None,
            verbose: verbose,
        }
    }

    pub fn init(&mut self, seed: u64) {
        self.random = Some(StdRng::seed_from_u64(seed))
    }

    pub fn push(&mut self, item: SrackValue, stdout: &mut dyn Write) {
        match &mut self.random {
            None => { panic!("Srack didn't initialized") }
            Some(r) => {
                let i = &r.gen_range(1..4);
                match i {
                    1 => {
                        if self.verbose {
                            writeln!(stdout, ">log: push").ok();
                        }

                        self.collection.push(item);
                    }
                    2 => {
                        if self.verbose { writeln!(stdout, ">log: do nothing").ok(); }
                    }
                    3 => {
                        if self.verbose {
                            writeln!(stdout, ">log: push x2").ok();
                        }
                        self.collection.push(item.clone());
                        self.collection.push(item);
                    }
                    4 => {
                        if self.verbose {
                            writeln!(stdout, ">log: pop & push").ok();
                        }
                        self.collection.pop();
                        self.collection.push(item);
                    }
                    &_ => { panic!("Invalid random value {i}") }
                }
            }
        }
    }

    pub fn poop(&mut self, stdout: &mut dyn Write) {
        match &mut self.random {
            None => { panic!("Srack didn't initialized") }
            Some(r) => {
                let count = *&r.gen_range(1..10);
                for _ in 0..count {
                    if self.verbose {
                        writeln!(stdout, ">log: Poop item").ok();
                    }
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