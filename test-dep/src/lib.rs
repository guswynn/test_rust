pub struct Gus {
    a: String,
}

impl Gus {
    fn do_it(&self) {}
}

struct Example {
    number: i32,
}

impl Example {
    fn boo() {
        println!("boo! Example::boo() was called!");
    }

    fn answer(&mut self) {
        self.number += 42;
    }

    fn get_number(&self) -> i32 {
        self.number
    }
}


pub trait Thing {
    fn self_sized(self) 
        where Self: Sized {}
}


#[derive(Default)]
pub struct Test {
    pub i: i64,
    pri: i64,
}
