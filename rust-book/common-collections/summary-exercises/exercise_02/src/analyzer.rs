pub fn analyze_the_first_letter_and_relocate_it_within_the_word(
    word: &String
) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    let mut new_word: String = String::new();

    chars.push('-');

    if check_if_the_letter_is_vowel(chars[0]) {
        chars.push('h');
    } else {
        chars.push(chars[0]);
    
        chars.remove(0);
    }

    chars.push('a');
    chars.push('y');

    for char in chars.iter() {
        new_word.push(*char)
    }
    
    return new_word;
}

fn check_if_the_letter_is_vowel(
    letter: char
) -> bool {
    let vowels: Vec<char> = vec![
        'a', 'e', 'i', 'o', 'u'
    ];

    for vowel in vowels {
        if vowel == letter {
            return true;
        }
    }

    return false;
}