// From the Onwership Inventory #1 quiz.
fn get_or_default2(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone()
    }
}

fn get_or_default3(arg: &mut Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.as_mut().unwrap();
    s.clone()
}

fn get_or_default4(arg: &Option<&str>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.to_string()
}

fn main() {}