/* 
fn main() {
    let v1: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
}

 */

/*  fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
} */

/* 
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
 */

fn main1() {
    let mut v = vec![100, 32];
    v.push(57);
    let mut i = 0;

    for n_ref in &mut v {
        // n_ref has type &mut i32
        *n_ref += 50;
        i += *n_ref;
    }

    println!("end: {}", i);
}

fn main() {
    let mut v = vec![1, 2, 3];
    let mut v2 = Vec::new();
    for i in &mut v {
      v2.push(i);
    }
    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a} {b}");
}