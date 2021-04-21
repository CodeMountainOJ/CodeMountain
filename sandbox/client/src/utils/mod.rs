pub fn join(vector: Vec<String>, delimeter: &str) -> String {
    if vector.len() == 0 {
        return String::from("");
    }

    let mut result = String::new();
    let mut idx: usize = 0;

    loop {
        if idx == vector.len() {
            break;
        }
        result += &vector[idx].clone();
        idx += 1;

        if idx < vector.len() {
            result += delimeter;
        }
    }

    result
}