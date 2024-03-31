use serde::ser::Error;

pub fn rm_delimeter(value: String, delimeter: String, replacment: String) -> String {
    let result: String;
    // split by delimeter
    let parts = value.split(&delimeter).collect::<Vec<_>>();
    result = parts.join(&replacment).to_string();

    // merge to new string
    result.to_string()
}

/// this function assumes that any "." denotes a file extension break
/// example - "some file.txt.bak" will become "some file"
/// example 2 = "some file.is cool.txt.bak" will become "some file"
pub fn rm_path_ext(value: String) -> String {
    let parts = value.split(".").collect::<Vec<_>>();
    parts.get(0).unwrap().to_string()
}

pub fn upper_first_letter(value: &mut str) -> Result<String, String> {
    if value.len() < 1 {
        return Err("Cannot parse an empty string as value".to_string());
    }
    let (left, _) = value.split_at_mut(1);
    left.make_ascii_uppercase();
    Ok(value.to_owned().clone())
}

pub fn upper_all_words_first_letters(value: String) -> Result<String, String> {
    if value.len() < 1 {
        return Err("Cannot parse an empty string".to_string());
    }
    let parts = value.split(" ").collect::<Vec<_>>();
    let mut new_parts: Vec<String> = vec!["".to_string(); parts.len()];
    for part_index in 0..parts.len() {
        let part = parts[part_index];
        let mut x = part.to_owned().clone();
        let val = upper_first_letter(&mut x).unwrap();
        new_parts[part_index] = val;
    }

    Ok(new_parts.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_removes_by_delimiter() {
        assert_eq!(
            rm_delimeter(
                "a_hog_hopped_a_log.txt".to_string(),
                "_".to_string(),
                " ".to_string(),
            ),
            "a hog hopped a log.txt"
        );
        assert_eq!(
            rm_delimeter(
                "a-hog-hopped-a-log.txt".to_string(),
                "-".to_string(),
                " ".to_string(),
            ),
            "a hog hopped a log.txt"
        );
        assert_eq!(
            rm_delimeter(
                "a-hog-hopped-a_log.txt".to_string(),
                "-".to_string(),
                " ".to_string(),
            ),
            "a hog hopped a_log.txt"
        );
        assert_eq!(
            rm_delimeter(
                "a-hog-hopped-a_log.txt".to_string(),
                "".to_string(),
                "".to_string(),
            ),
            "a-hog-hopped-a_log.txt"
        );
        assert_eq!(
            rm_delimeter("".to_string(), "".to_string(), "".to_string()),
            ""
        );
        assert_eq!(
            rm_delimeter("".to_string(), "_".to_string(), "-".to_string()),
            ""
        );
    }

    #[test]
    fn it_removes_file_extention() {
        assert_eq!(
            rm_path_ext("a_hog_hopped_a_log.txt".to_string()),
            "a_hog_hopped_a_log"
        );
        assert_eq!(
            rm_path_ext("a hog hopped.a log.bak.txt".to_string()),
            "a hog hopped"
        );
        assert_eq!(rm_path_ext(".txt".to_string()), "");
    }

    #[test]
    fn it_uppercases_first_letter_of_word() {
        assert_eq!(
            upper_first_letter(&mut "banana".to_owned()).unwrap(),
            "Banana"
        );
        assert_eq!(
            upper_first_letter(&mut "".to_owned()).unwrap_err(),
            "Cannot parse an empty string as value"
        );
        assert_eq!(upper_first_letter(&mut "Bang".to_owned()).unwrap(), "Bang");
        assert_eq!(
            upper_first_letter(&mut "_banana".to_owned()).unwrap(),
            "_banana"
        );
    }

    #[test]
    fn it_uppercases_first_letter_of_every_word() {
        assert_eq!(
            upper_all_words_first_letters("triston is cool".to_string()).unwrap(),
            "Triston Is Cool"
        );
        assert_eq!(
            upper_all_words_first_letters("".to_string()).unwrap_err(),
            "Cannot parse an empty string"
        );
        assert_eq!(
            upper_all_words_first_letters("Triston Is Cool".to_string()).unwrap(),
            "Triston Is Cool"
        );
    }
}
