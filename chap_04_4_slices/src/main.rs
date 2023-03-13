/* fn first_word1(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if [b' ', b',', b'.'].contains(&item) {
            return &s[0..i];
        }
    }

    &s[..]
}
 */

/* 
fn main() {
    println!("{}", first_word2(&String::from("hello, i am a sentence")));
}
 */

/* 
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first word is: {}", word);
}
 */

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if [b' ', b',', b'.'].contains(&item) {
            return &s[0..i];
        }
    }

    &s[..]
}

 fn main1() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn main() {
    println!(
      "&String={} &str={}",
      std::mem::size_of::<&String>(),
      std::mem::size_of::<&str>(),
    );

    for (i, by3, by5) in (1..=100).into_iter().map(|i| {(i, i % 3 == 0, i % 5 == 0)}) {
        print!("{}: ", i);
        if by3 && by5 {
            println!("FizzBuzz!");
        } else if by3 {
            println!("Fizz");
        } else if by5 {
            println!("Buzz");
        } else {
            println!("No!");
        }
    }
}