fn main() {

    let s2 = String::from("       helloworldhow   ");
    println!("first word was {}", first_word(&s2));
    println!("string was {s2}");

}

fn first_word(s: &String) -> String {
    let mut in_word: bool = false;
    let mut word = String::new();

    for (_i, item) in s.chars().enumerate() {
        if in_word {
            if item == ' ' {
                break;
            } else {
                in_word = true;
                word.push(item);
            }
        } else {
            if item == ' ' {
                continue;
            } else {
                in_word = true;
                word.push(item);
            }
        }
    }
    return word;
}

// fn do_something(s: String) {
//     println!("in do something {s}");
// }

// fn do_a_different_thing(s: &mut String) {
//     s.push_str(", asdasd");
//     println!("different thing {s}");
// }

// fn do_something_with_ref(s: &mut String) {
//     s.push_str(", world!");
//     println!("in do something red {s}");
// }