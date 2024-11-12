
pub fn reverse(input: &str) -> String {
    if input.is_empty() {
        return String::new()
    }
    let mut result = String::new();
    for i in input.chars().rev() {
        result.push(i);
    } 
    result
}
