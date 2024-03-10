pub fn reverse_words(input: String) -> String {
    let parts = input.split(" ");
    let mut words = Vec::new();

    for part in parts {
        let trimmed = part.trim();

        if trimmed.len() != 0 {
            words.push(trimmed);
        }
    }

    words.reverse();

    let joined = words.join(" ");
    return joined;
}

#[cfg(test)]
mod reverse_words_tests {
    use crate::reverse_words::reverse_words;

    #[test]
    fn example1() {
        let input = "the sky is blue".to_string();
        let expected = "blue is sky the".to_string();

        let result = reverse_words(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let input = "  hello world  ".to_string();
        let expected = "world hello".to_string();

        let result = reverse_words(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn example3() {
        let input = "a good   example".to_string();
        let expected = "example good a".to_string();

        let result = reverse_words(input);
        assert_eq!(result, expected);
    }
}