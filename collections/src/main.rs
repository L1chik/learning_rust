use std::collections::HashMap;

fn main() {
    // vector1();
    // vector2();
    // vector3();
    // vector4();
    // vector5();

    // string();
    // string2()
    // string3();

    // hashmap();
    // hashmap2();
    // hashmap3();
    // hashmap4();
    // hashmap5();
    // hashmap6();
    // hashmap7();
    hashmap8();
}

fn vector1() {
    let v:Vec<i32> = Vec::new();

    let a = vec![1, 2, 3];

    let mut b = Vec::new();
    b.push(5);
    b.push(6);

    println!("{:?}", b)
}

fn vector2() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("element is {}", third);

    match v.get(2) {
        Some(third) => println!("element is {}", third),
        None => println!("no elemrnt"),
    }
}

fn vector3() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
}

// fn vector4() {
//     let mut v = vec![1, 2, 3, 4, 5];
//
//     let first = &v[0]; //неизменяемые ссылка на нулевой элемент
//
//     v.push(6); //изменение вектора приведёт к ошибке,
//     // потому что для вектора может выделиться новый участок памяти
//     // и ссылку на нулевой элемент возможно надо будет поменять
//
//     println!("First el is {}", first)
// }

fn vector5() {
    let mut v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{}", i);
    // }

    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}


fn vector6() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn string() {
    let mut s = String::new();

    let data = "initial test";

    s = data.to_string();
    s = "initial xyeta".to_string();

    s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);
}

fn string2() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("s2 is {}", s2);

    let mut a = String::from("lo");
    a.push('h');

    println!("{}", a);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    println!("{}", s3)
}

fn string3() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{}", s)
}

fn hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

fn hashmap2() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);
}

fn hashmap3() {
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // for i in field_value.chars(){
    //     println!("{}", i);
    // }
}

fn hashmap4() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);
}

fn hashmap5() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"),  50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

//Перезапись значений
fn hashmap6() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); //Выведет 25
}

fn hashmap7() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn hashmap8() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
