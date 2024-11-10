//! This module contains functions to insert matches (captutes) in target path.
use crate::errors::MassMoveError;

/// Private function to insert match in filename.
/// Function insert concrete match in filename and return it.
/// If match not exist in matches or position is invalid, function return MassMoveError::InvalidTargetPath.
fn insert_match_in_filename(
    matches: &[String],
    mut filename: String,
    i: usize,
) -> Result<String, MassMoveError> {
    if i != 0 && i <= matches.len() {
        filename.push_str(&matches[i - 1]);
        Ok(filename)
    } else {
        Err(MassMoveError::InvalidTargetPath(format!(
            "position #{i} not exist in source path",
        )))
    }
}

/// Public function to insert matches(captures) in target path.
/// Usage:
/// ```rust
/// let matches = vec!["v1".to_string(), "2024".to_string()];
/// let target = "file-#1-#2.txt";
/// let result = insert_matches_in_target(&matches, target)?;
/// assert_eq!(result, "file-v1-2024.txt");
/// ```
pub fn insert_matches_in_target(
    matches: &[String],
    pattern: &str,
) -> Result<String, MassMoveError> {
    let mut new_filename = String::new();

    let mut is_matching = false;
    let mut match_index = 0;

    for c in pattern.chars() {
        if c != '#' && !is_matching {
            new_filename.push(c);
            continue;
        }

        if is_matching && c.is_ascii_digit() {
            match_index = match_index * 10 + c.to_digit(10).unwrap() as usize;
            continue;
        }

        if c == '#' {
            if is_matching {
                new_filename = insert_match_in_filename(matches, new_filename, match_index)?;
            }
            is_matching = true;
            match_index = 0;
            continue;
        }

        if is_matching {
            new_filename = insert_match_in_filename(matches, new_filename, match_index)?;
            is_matching = false;
            match_index = 0;
        }

        new_filename.push(c)
    }

    if is_matching {
        new_filename = insert_match_in_filename(matches, new_filename, match_index)?;
    }

    Ok(new_filename)
}

#[test]
fn test_insert_matches_in_target_pattern() {
    let cases: Vec<(&str, Vec<String>, &str, bool)> = vec![
        (
            "file-#1-v1.txt",
            vec!["1".to_string()],
            "file-1-v1.txt",
            true,
        ),
        (
            "file#1.txt",
            vec!["_match1".to_string()],
            "file_match1.txt",
            true,
        ),
        (
            "file#1#2.txt",
            vec!["_match1".to_string(), "_match2".to_string()],
            "file_match1_match2.txt",
            true,
        ),
        (
            "#1",
            vec!["some_some.txt".to_string()],
            "some_some.txt",
            true,
        ),
        ("file_#1.#2.txt", vec!["value1".to_string()], "", false),
    ];
    for case in cases {
        let result = insert_matches_in_target(&case.1, &case.0);
        assert_eq!(
            result.is_ok(),
            case.3,
            "pattern: {}, expected is_ok() = {}, but result is {}",
            case.0,
            case.3,
            result.is_ok(),
        );
        if result.is_ok() {
            assert_eq!(result.unwrap(), case.2)
        }
    }
}
