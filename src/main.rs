use crate::reverse_words::reverse_words;
mod reverse_words;

fn main() {
    println!("{}", reverse_words("the sky is blue".to_string()));
    println!("{}", reverse_words("  hello world  ".to_string()));
    println!("{}", reverse_words("a good   example".to_string()));
}
