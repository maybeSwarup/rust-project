use std::fs;

fn main() {
    let res = read_from_file_unsafe("example.txt".to_string());

    let result_string = match res {
        Ok(result) => result,
        Err(error) => error,
    };
    println!("file content: {}", result_string);
}

fn read_from_file_unsafe(file_path: String) -> Result<String, String> {
    let res = fs::read_to_string(file_path);

    if let Ok(content) = res {
        return Ok(content);
    } else {
        return Err("Error reading file".to_string());
    }
}
