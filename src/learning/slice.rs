
pub fn find_first_word(word: &str)->&str{
    let mut space_index = 0;
    for (_, i) in word.chars().enumerate(){
        if i == ' ' {
            break;
        }
        space_index = space_index + 1;
    }
    return &word[0..space_index];
}