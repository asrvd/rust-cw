// return number of vowels in a given string

pub fn get_vowel_count(string: &str) -> usize {
    string.chars().filter(|c| "aeiou".contains(*c)).count()
}
