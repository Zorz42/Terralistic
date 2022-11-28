use std::collections::HashSet;


#[test]
fn test_get_resource_path(){
    assert_eq!(get_resource_path(
            String::from(r"/home/nejc/CLionProjects/Terralistic/cmake-build-debug/šwergkćwergžü/Terralistic")),
            String::from(r"/home/nejc/CLionProjects/Terralistic/cmake-build-debug/šwergkćwergžü/Resources/"), "test 0 failed");
    assert_eq!(get_resource_path(
            String::from(r"C:\Users\Uporabnik\CLionProjects\Terralistic\cmake-build-debug\šwergkćwergžü\Terralistic.exe")),
            String::from(r"C:\Users\Uporabnik\CLionProjects\Terralistic\cmake-build-debug\šwergkćwergžü/Resources/"), "test 1 failed");
    assert_eq!(get_resource_path(String::from(r"/test/folder/exec")), String::from(r"/test/folder/Resources/"), "test 2 failed"); //previously used tests
    assert_eq!(get_resource_path(String::from(r"/test/folder/MacOS/exec")), String::from(r"/test/folder/Resources/"), "test 3 failed");
    assert_eq!(get_resource_path(String::from(r"/testing/1234/exec")), String::from(r"/testing/1234/Resources/"), "test 4 failed");
    assert_eq!(get_resource_path(String::from(r"/testing/1234/MacOS/exec")), String::from(r"/testing/1234/Resources/"), "test 5 failed");
    assert_eq!(get_resource_path(
            String::from(r"/folder/folder/folder/folder/folder/folder/notfolder")),
            String::from(r"/folder/folder/folder/folder/folder/folder/Resources/"), "test 6 failed");
    assert_eq!(get_resource_path(
            String::from(r"/folder/folder/folder/folder/šwćgregeropgjćšwž/folder/folder/MacOS/notfolder")),
            String::from(r"/folder/folder/folder/folder/šwćgregeropgjćšwž/folder/folder/Resources/"), "test 7 failed");
    assert_eq!(get_resource_path(String::from(r"./exec")), String::from(r"./Resources/"), "test 8 failed");
    assert_eq!(get_resource_path(String::from(r"exec")), String::from(r"./Resources/"), "test 9 failed");
}




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