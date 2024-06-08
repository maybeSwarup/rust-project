fn find_first_a(s: String) -> Option<usize> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index);
        }
    }
    return None
}

fn main() {
    let my_string = String::from("remun");
    let res = find_first_a(my_string);

    match res {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not foudn in the string."),
    }
}