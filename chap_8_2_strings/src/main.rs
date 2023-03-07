/* 
fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
}
 */

/* 
fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
 */

/* 
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
}
 */

/* 
/// `push_str` does not take ownership of its argument, so it can be reused.
 fn main() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    s1.push('x');
    println!("s1 is {}", s1);
}
 */

/* 
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // note s1 has been moved here and can no longer be used
    //println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
}
 */

/* 
/// First option, with `Add` trait
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
}
 */

/* 
/// Second option, with `format!`
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}
 */

/* 
/// `String` does not implement `Index<u>`, where `u` is an integral type.
/// The below will error.
fn main() {
    let s1 = String::from("hello");
    let h = s1[0];
}
 */

/* 
#![allow(unused)]
/// Using slices improperly with UTF-8 strings will cause runtime errors.
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
}
 */

#![allow(unused)]
fn main() {
    //for b in "Здравствуйте".chars()
    for b in "Здравствуйте".bytes() {
        println!("{}", b);
    }
}