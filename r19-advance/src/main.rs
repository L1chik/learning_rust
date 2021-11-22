use std::slice;
use std::fmt;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    // unsafe_test();
    // unsafe_test2();
    //
    // unsafe_test3();
    //
    // unsafe_test4();
    //
    // trait_advance();

    r19::trait_test2();
    r19::trait_test3();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

fn unsafe_test() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // let address = 0x012345usize;
    // let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn unsafe_test2() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
    }
}

fn unsafe_test3() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!")
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn unsafe_test4() {
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// unsafe trait Foo {
//
// }
//
// unsafe impl Foo for i32 {
//
// }

fn trait_advance() {
    assert_eq!(
        r19::Point { x: 1, y: 0 } + r19::Point { x: 2, y: 3},
        r19::Point { x: 3, y: 3}
    );
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}