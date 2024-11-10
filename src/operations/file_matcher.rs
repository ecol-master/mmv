use crate::errors::MassMoveError;
use regex::Regex;
use regex_syntax::escape;
use std::fs::{self, ReadDir};
use std::path::{Path, PathBuf};

/// `FileMatcher` is a struct that matches files in a directory based on a pattern and returns a vector of `FileWithMatches`
/// `FileMatcher` returns an error is no one file is matched for a pattern or if the source path is invalid.
/// Usage:
/// ```rust
/// use crate::operations::file_matcher::FileMatcher;
///
/// let example_source_path = "./path/to/*.txt"
/// let matcher = FileMatcher::from_source_path(example_source_path)?;
/// let files_with_matches = matcher.get_files_with_matches()?;
/// ```
pub struct FileMatcher {
    source_pattern: String,
    source_directory: String,
}

/// `FileWithMatches` is a struct that contains a filepath and a vector of matches for a file.
/// Example of file_with_matches for a concrete file and pattern:
/// ```rust
/// use crate::operations::file_matcher::FileWithMatches;
///
/// let pattern = "file-*.*";
///
/// let file = FileWithMatches {
///    filepath: "./path/to/file-new.txt".to_owned(),
///    matches: vec!["new".to_owned(), "txt".to_owned()],
/// };
/// ```
pub struct FileWithMatches {
    filepath: String,
    matches: Vec<String>,
}

pub type FileMatcherResult = Vec<FileWithMatches>;

impl FileMatcher {
    /// Construct a new `FileMatcher` from a source path.
    /// Source path is a first command line argument.
    pub fn from_source_path(source_path: PathBuf) -> Result<Self, MassMoveError> {
        let file_name = source_path.file_name();
        let parent = source_path.parent();

        if file_name.is_none() || parent.is_none() {
            return Err(MassMoveError::InvalidSourcePath(
                source_path.to_str().unwrap().to_owned(),
            ));
        }

        Ok(Self {
            source_pattern: file_name.unwrap().to_str().unwrap().to_owned(),
            source_directory: parent.unwrap().to_str().unwrap().to_owned(),
        })
    }

    /// Function format input pattern to valid regex pattern.
    /// It screens all speacial characters and then make from '*' a capture group.
    fn pattern_to_regex(&self) -> String {
        let escaped = escape(&self.source_pattern);
        let regex_pattern = escaped.replace("\\*", "([^.]*)");
        format!("^{}$", regex_pattern)
    }

    /// Function checks if file matches the pattern.
    fn is_file_match_pattern(&self, filename: &str) -> Result<bool, MassMoveError> {
        let pattern = self.pattern_to_regex();
        Ok(Regex::new(&pattern).unwrap().is_match(filename))
    }

    /// Function returns a vector of all matches for a file.
    fn get_file_matches(&self, filename: &str) -> Result<Vec<String>, MassMoveError> {
        let mut matches = Vec::new();
        let re = Regex::new(&self.pattern_to_regex()).unwrap();

        for caps in re.captures_iter(filename) {
            for (i, cap) in caps.iter().enumerate() {
                if let Some(cap) = cap {
                    if i != 0 && cap.start() != cap.end() {
                        matches.push(filename[cap.start()..cap.end()].to_owned());
                    }
                }
            }
        }

        Ok(matches)
    }

    /// Function try to read a source directory and return a std::fs::ReadDir object.
    fn read_source_directory(&self) -> Result<ReadDir, MassMoveError> {
        let mut read_path = self.source_directory.clone();
        if read_path.is_empty() {
            read_path = "./".to_owned();
        }
        match fs::read_dir(read_path) {
            Ok(read_directory) => Ok(read_directory),
            Err(_) => Err(MassMoveError::DirectoryNotFound(
                self.source_directory.clone(),
            )),
        }
    }

    /// Function collects all matched files from a source directory that match the pattern.
    fn collect_matched_files(&self) -> Result<Vec<String>, MassMoveError> {
        let mut files = Vec::new();
        let directory = self.read_source_directory()?;

        for entry in directory.into_iter().filter_map(|e| e.ok()) {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let filename = entry.file_name().into_string().unwrap();
                    if self.is_file_match_pattern(&filename)? {
                        files.push(filename);
                    }
                }
            }
        }

        if files.is_empty() {
            Err(MassMoveError::NoFilesForPattern(String::from(
                &self.source_pattern,
            )))
        } else {
            Ok(files)
        }
    }

    /// Function returns a vector of `FileWithMatches` for all files that match the pattern.
    pub fn get_files_with_matches(&self) -> Result<FileMatcherResult, MassMoveError> {
        let mut result = Vec::new();

        for file in self.collect_matched_files()? {
            let matches = self.get_file_matches(&file)?;
            let filepath = Path::new(&self.source_directory)
                .join(file)
                .to_str()
                .unwrap()
                .to_owned();
            result.push(FileWithMatches { matches, filepath })
        }
        Ok(result)
    }
}

impl FileWithMatches {
    pub fn filename(&self) -> &str {
        &self.filepath
    }

    pub fn matches(&self) -> &Vec<String> {
        &self.matches
    }
}

#[test]
fn test_match_initizalizer() {
    let cases: Vec<(&str, &str, &str, bool)> = vec![
        ("./path/to/file.png", "file.png", "./path/to", true),
        ("./path/to/file.txt", "file.txt", "./path/to", true),
        ("./path/to/file", "file", "./path/to", true),
        ("./path/to/file/*.png", "*.png", "./path/to/file", true),
        ("./file", "file", ".", true),
        ("file", "file", "", true),
        ("/file", "file", "/", true),
        ("/path/to/file", "file", "/path/to", true),
        ("/path/to/file.txt", "file.txt", "/path/to", true),
        ("file", "file", "", true),
        ("file.txt", "file.txt", "", true),
        ("file.png", "file.png", "", true),
        ("", "", "", false),
        ("path/to/", "to", "path", true),
    ];

    for case in cases {
        let matcher = FileMatcher::from_source_path(PathBuf::from(case.0));
        assert_eq!(
            matcher.is_ok(),
            case.3,
            "wrong initialize for source_path: \"{}\"",
            case.0
        );
        if !case.3 {
            continue;
        }

        let matcher = matcher.unwrap();
        assert_eq!(matcher.source_pattern, case.1, "failed for: {}", case.0);
        assert_eq!(matcher.source_directory, case.2, "failed for: {}", case.0);
    }
}

#[test]
fn test_file_match_pattern() {
    let cases: Vec<(&str, &str, bool)> = vec![
        ("file.png", "*.png", true),
        ("file.txt", "*.png", false),
        ("file", "file", true),
        ("file", "file.txt", false),
        ("file.txt", "file.txt", true),
        ("file.txt", "file", false),
        ("file.txt", "*.txt", true),
        ("file.txt", "*.png", false),
        ("file.txt", "file.*", true),
        ("backend.tar", "*.*", true),
    ];

    for case in cases {
        let file_matcher = FileMatcher {
            source_directory: String::from("./"),
            source_pattern: String::from(case.1),
        };

        assert_eq!(
            file_matcher.is_file_match_pattern(case.0).unwrap(),
            case.2,
            "file: {}, pattern: {}",
            case.0,
            case.1
        );
    }
}

#[test]
fn test_get_file_matches() {
    let cases: Vec<(&str, &str, Vec<String>)> = vec![
        (
            "*file*.png",
            "some_file_1.png",
            vec![String::from("some_"), String::from("_1")],
        ),
        ("*file*.png", "file_2.png", vec![String::from("_2")]),
        ("*file*.png", "file.png", vec![]),
        (
            "*.*",
            "backend.tar",
            vec![String::from("backend"), String::from("tar")],
        ),
    ];

    for case in cases {
        let matcher = FileMatcher {
            source_directory: "./".to_owned(),
            source_pattern: case.0.to_owned(),
        };
        assert_eq!(
            matcher.get_file_matches(case.1).unwrap(),
            case.2,
            "failed get matches for: \"{}\"",
            case.1
        );
    }
}
