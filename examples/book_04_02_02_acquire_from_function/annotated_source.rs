fn main() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
let s1 = String::from("hello");

let len = calculate_length(&s1);
}