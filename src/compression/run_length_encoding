pub fn rle_encode(input: &str) -> &str {
    let mut result = String::new();
    let mut count = 1;

    let chars = input.chars().collect::<Vec<char>>();
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            result.push(chars[i - 1]);
            result.push_str(&count.to_string());
            count = 1;
        }
    }
    result
}



pub fn rle_decode(input : &str) -> &str{
  
        let mut result = String::new();
        let mut count = String::new(); // To accumulate the count
    
        let mut chars = input.chars().peekable(); // Create an iterator with peek support
    
        while let Some(current_char) = chars.next() {
            if let Some(next_char) = chars.peek() {
                if next_char.is_digit(10) {
                    // If the next character is a digit, accumulate it
                    count.push(*next_char);
                } else {
                    // Otherwise, append the current character (repeated by the count)
                    let repeat_count = count.parse::<usize>().unwrap_or(1);
                    result.push_str(&current_char.to_string().repeat(repeat_count));
                    count.clear(); // Reset the count
                }
            } else {
                // If there's no next character, append the current character once
                result.push(current_char);
            }
        }
    
        result
}
    
  