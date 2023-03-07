fn main1() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

fn main2() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn main3() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // println!("{}: {}", field_name, field_value);
}

fn main4() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    println!("name: {}", scores["Yellow"]);
}

fn main5() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

/// Exercises

/// 1

/// Median of a vector; modifies its input.
fn median(v : &mut Vec<u32>) -> Option<f32> {
    let len = v.len();
    if len == 0 {
        return None;
    }

    v.sort();
    let median;
    if len % 2 == 0 {
        median = ((v[len / 2] + v[(len / 2) + 1]) as f32) / 2.0
    } else {
        median = v[(len + 1) / 2] as f32;
    }

    Some(median)
}

/// Median of a vector, without having to sort the vector (but still modifying it).
fn median2(v : &mut Vec<u32>) -> Option<f32> {
    let len = v.len();
    if len == 0 {
        return None;
    }

    let median;
    if len % 2 == 0 {
        let left = *v.select_nth_unstable(len / 2).1;
        let right = *v.select_nth_unstable((len / 2) + 1).1;
        median = ((left + right) as f32) / 2.0;
    } else {
        median = *v.select_nth_unstable((len + 1) / 2).1 as f32;
    }

    Some(median)
}

/// Median of a vector, without needing to modify it.
fn median3(v: &Vec<u32>) -> Option<f32> {
    let len = v.len();
    if len == 0 {
        return None;
    }

    use std::collections::HashMap;

    let mut median : f32;
    let mut rel_freqs : HashMap<u32, f32> = HashMap::new();
    for n in v.into_iter() {
        let count = rel_freqs.entry(*n).or_insert(0.0);
        *count += 1.0;
    }
    for (_, val) in rel_freqs.iter_mut() {
        *val /= len as f32;
    }

    // As soon as we find an element whose relative frequency puts
    // this value above 0.5 (50%), but below 50% without, we've found the median.
    let mut so_far = 0.0;

    let mut keys_vals = rel_freqs.into_iter().collect::<Vec<_>>();
    keys_vals.sort_by_key(|tup| (*tup).0);

    for (key, val) in keys_vals {
        if so_far < 0.5 && val + so_far >= 0.5 {
            return Some(key as f32)
        }
        so_far += val;
    }

    None
}

/// 2
/// Convert a word in the Latin alphabet to pig latin.

fn pig_latinize(word: &String) -> String {
    let mut res = String::new();
    let mut suffix = String::new();

    // The variable doesn't need to be initialized, but not doing so
    // results in a compiler warning about possible uninitialized values.
    let mut fst_chr_is_vowel = false;

    let mut iter = word.chars().peekable();
    if let Some(c) = iter.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                res.push(c.to_ascii_lowercase());
                suffix.push_str("-hay");
                fst_chr_is_vowel = true;
            },
            'a'..='z' | 'A'..='Z' => {
                suffix.push('-');
                suffix.push(c.to_ascii_lowercase());
                suffix.push_str("ay");
            }
            _ => {
                res.push(c.to_ascii_lowercase());
            }
        };
    };

    // Upper the second letter, first in the new word, if a suffix has been added.
    if !fst_chr_is_vowel {
        // This needs to be nested, as otherwise the compiler warns that
        // `if let` combined with regular `if` expressions are an unstable feature.
        if let Some(c) = iter.next() {
            res.push_str(&c.to_uppercase().to_string());
        }
    }

    while let Some(c) = iter.next() {
        res.push(c);
    }

    res + &suffix
}


/* 
fn main() {
    use std::collections::HashMap;

    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(i);
    }
    let mut sum = 0;
    for i in h.get(&'l').unwrap() {
        sum += *i;
    }
    println!("{}", sum);
}
 */

fn main() {
    let mut v = vec![1, 2, 9, 5, 3, 8, 6, 5, 3, 4, 1, 3, 1, 5, 6, 3, 8, 9, 7];
    v.reverse();

    println!("the vector's median is: {:?}", median(&mut v));
    println!("the vector's median2 is: {:?}", median2(&mut v));
    println!("the vector's median3 is: {:?}", median3(&v));

    let word1 = String::from("apple");
    let word2 = String::from("Kílìmãñjärô");
    let word3 = String::from("Ubiquity");
    println!("word: {}; pig-latin: {}", word3, pig_latinize(&word3));
}