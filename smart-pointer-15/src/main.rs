use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::ops::Deref;
// use crate::List2::Cons as OtherCons;
fn main() {
    // let b = Box::new(5);
    // println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // let list = Cons(1, Box::new(Cons(
    //     2, Box::new(Cons(3, Box::new(Nil))))));

    deref_op();
    deref_op2();

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    smart_pointer_drop();

    // rc_pointer();

    new_test();
}

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

fn deref_op() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

}

fn deref_op2() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn deref_op3() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn smart_pointer_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
//
// fn rc_pointer() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     println!("count after creating a = {}", Rc::strong_count(&a));
//     let b = Cons(3, Rc::clone(&a));
//     println!("count after creating b = {}", Rc::strong_count(&a));
//     {
//         let c = Cons(4, Rc::clone(&a));
//         println!("count after creating c = {}", Rc::strong_count(&a));
//     }
//     println!("count after c goes out of scope = {}", Rc::strong_count(&a));
// }

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// use crate::List::{Cons, Nil};
use std::cell::{Ref, RefCell};
// use std::rc::Rc;

fn new_test() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

