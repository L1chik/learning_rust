#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Fail test");
    // }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(result.contains("Carol"));
    // }
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    #[test]
    // Для примеров 1 и 2
    // #[should_panic]

    // Для примера №3
    #[should_panic(expected="Guess value must be less than or equal to 100")]

    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2+2==4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }


}


/* Макрос assert! для сравнения результата тестируемого кода и ожидаемого значения
assert_eq! & assert_ne! эти макросы сравнивают два аргумента на равенство илил неравенство

 */

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        // Пример №1
        // if value < 1 || value > 100 {
        //     panic!("Guess .. {}", value)
        // }

        // Пример №2
        // if value < 1 {
        //     panic!("Guess ... {}", value)
        // }

        // Пример №3
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}."
                , value);
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}."
                , value
            );
        }

        Guess {
            value
        }
    }
}


