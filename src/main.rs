use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

fn main() {
    work_13();
}


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}



#[allow(unused)]
fn work_13() {
    let value = Rc::new(RefCell::new(5));
    //5 -> Nil
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    //6 -> 5 ->  Nil
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    //10 -> 5 -> Nil
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    //获取value的可变借用，将其加10，注意：现在有多个Rc指向value
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
