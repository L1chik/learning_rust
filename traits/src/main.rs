fn main() {
    // code2();
    // code3();

    // points();
    // points2();
    // points3();
    points4();
}


fn code() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("{}", largest)
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largecst_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn code2() {
    let number_list = vec![34, 50 ,23, 100, 54];

    let result = largest_i32(&number_list);
    println!("{}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("{}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largecst_char(&char_list);
    println!("{}", result);

}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn code3() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }


struct Point<T> {
    x: T,
    y: T,
}

fn points() {
    let integer = Point{x: 5, y: 10};
    let float = Point{x: 1.0, y: 2.0};

    // let smth = Point{x: 1, y: 2.9}; //так быть не может
}

struct Points<T, U> { //говорим о том, что поля могут быть разных тпипов
    x: T,
    y: U,
}

fn points2() {
    let both_int = Points {x: 1, y: 2};
    let both_float = Points {x: 1.0, y: 2.0};
    let int_and_float = Points {x: 1.0, y:10};
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn points3() {
    let p = Point {x:5, y:10};

    println!("p.x = {}", p.x());
}

impl<T, U> Points<T, U> {
    fn mixup<V, W>(self, other: Points<V, W>) -> Points<T, W> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}

fn points4() {
    let p1 = Points {x:5, y: 10.4};
    let p2 = Points {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}