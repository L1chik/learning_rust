use r17oop::AveragedCollection;
use r17oop::Draw;
use r17oop::{Button, Screen};

fn main() {
    // test();
    // test2();



}

fn test() {
    let mut new = AveragedCollection::new(vec![1, 2, 3], 2.);
    AveragedCollection::add(&mut new, 4);
    AveragedCollection::remove(&mut new);
    println!("{:?}", AveragedCollection::average(&new))
}

fn test2() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    // let screen = Scree {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    screen.run();
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        todo!();
    }
}
