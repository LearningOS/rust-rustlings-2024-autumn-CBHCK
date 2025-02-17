// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.



// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut uppercase = match c.next() {
        None => String::new(),
        Some(first) => first.to_string().to_ascii_uppercase(),
    };
    loop{
		//此处是重点：调用了next()这个方法
		let case= match c.next(){
		None=>break,
        Some(case)=>case};

        uppercase = uppercase + case.to_string().as_str();
    };
    uppercase
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut iter_words = words.iter();
    let mut vec_string = vec![];
    loop {
        let substring = match iter_words.next(){
            None=>break,
            Some(case)=>capitalize_first(case)};

        vec_string.push(substring);
    }
    vec_string 
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut vec2string = String::new();
    let vec_words = capitalize_words_vector(words);
    for i in vec_words
    {
        vec2string = vec2string + i.as_str();
    }
    vec2string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
