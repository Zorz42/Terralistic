use std::collections::HashSet;

/*
returns path from beginning to last / or \\
 */
fn remove_end_until_chars(string: String, chars: HashSet<char>) -> String {
    let mut char_vec: Vec<char> = string.chars().collect();//split string to safely use utf8
    while !char_vec.is_empty() && !chars.contains(&char_vec[char_vec.len() - 1]) {
        char_vec.pop();
    }
    if !char_vec.is_empty() {
        char_vec.pop();
    }
    char_vec.into_iter().collect()
}

/*
returns path from last / or \\ until its end
 */
fn return_end_until_chars(string: String, chars: HashSet<char>) -> String {
    let char_vec: Vec<char> = string.chars().collect();//split string to safely use utf8
    let mut iter = char_vec.len() - 1;
    while iter > 0 as usize && !chars.contains(&char_vec[iter]) {
        iter -= 1;
    }
    char_vec[iter + 1..char_vec.len()].into_iter().collect()
}

/*
generates path to resources from executable path end returns it
 */
pub fn get_resource_path(executable_path: String) -> String {
    let mut parent_directory = remove_end_until_chars(executable_path, HashSet::from(['/', '\\']));

    if parent_directory.is_empty() {
        parent_directory.push('.');
    }

    let parent_parent_directory = remove_end_until_chars(parent_directory.clone(), HashSet::from(['/', '\\']));
    let parent_directory_name = return_end_until_chars(parent_directory.clone(), HashSet::from(['/', '\\']));

    if parent_directory_name == "MacOS" {
        parent_parent_directory + "/Resources/"
    }else {
        parent_directory + "/Resources/"
    }
}