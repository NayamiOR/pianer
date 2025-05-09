use crate::SearchResultEntry;

fn print_schema() {
    let schema = serde_json::to_string(&SearchResultEntry {
        keyword: "example".to_string(),
        file_name: "example.txt".to_string(),
        file_path: "/path/to/example.txt".to_string(),
    })
        .unwrap();
    println!("{}", schema); // 输出: {"percentage":0.5}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_schema() {
        print_schema();
    }
}
