use std::io;

fn main() {
    loop {
        println!("Input text to convert:");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let mut trimmed = line.trim().to_string();
        if trimmed == "quit" {
            break;
        }

        piglatinize(&mut trimmed);
        println!("{}", trimmed)

    }
}


fn piglatinize(s: &mut String) {
    let first_letter = s.remove(0);
    if !is_fowel(first_letter){
        s.push(first_letter);
        s.push_str("-ay");
    } else {
        s.push_str("-hay");
    }

}


fn is_fowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}