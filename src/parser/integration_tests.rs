
#[cfg(test)]
mod tests {
    use crate::parser::parse_file;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_basic_programs() {
        let basic_programs_dir = Path::new("programs").join("basic");
        for entry in fs::read_dir(basic_programs_dir).expect("Directory not found") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_file() {
                let content = fs::read_to_string(&path).expect("Failed to read file");
                let parsed = parse_file(&content);
                assert!(parsed.is_ok(), "Failed to parse file: {:?}\n {:#?}", path, parsed);
            }
        }
    }
}
