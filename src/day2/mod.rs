#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod a {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    pub fn find_score_a(file_path: PathBuf) -> usize {
        BufReader::new(File::open(file_path).unwrap())
            .lines()
            .fold([0, 0], |[s1, s2], round| match round.unwrap().as_str() {
                "A X" => [4 + s1, 3 + s2],
                "A Y" => [8 + s1, 4 + s2],
                "A Z" => [3 + s1, 8 + s2],
                "B X" => [1 + s1, 1 + s2],
                "B Y" => [5 + s1, 5 + s2],
                "B Z" => [9 + s1, 9 + s2],
                "C X" => [7 + s1, 2 + s2],
                "C Y" => [2 + s1, 6 + s2],
                "C Z" => [6 + s1, 7 + s2],
                _ => [s1, s2],
            })[0]
    }
}

pub mod b {
    use std::collections::BTreeSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    pub fn find_score_b(file_path: PathBuf) -> usize {
        BufReader::new(File::open(file_path).unwrap())
            .lines()
            .fold([0, 0], |[s1, s2], round| match round.unwrap().as_str() {
                "A X" => [4 + s1, 3 + s2],
                "A Y" => [8 + s1, 4 + s2],
                "A Z" => [3 + s1, 8 + s2],
                "B X" => [1 + s1, 1 + s2],
                "B Y" => [5 + s1, 5 + s2],
                "B Z" => [9 + s1, 9 + s2],
                "C X" => [7 + s1, 2 + s2],
                "C Y" => [2 + s1, 6 + s2],
                "C Z" => [6 + s1, 7 + s2],
                _ => [s1, s2],
            })[1]
    }
}

mod tests {
    use crate::day2::a::find_score_a;
    use crate::day2::b::find_score_b;

    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day2/input_test.txt");

        assert_eq!(find_score_a(d.to_owned()), 15);
    }

    #[test]
    fn test_a() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day2/input.txt");

        assert_eq!(find_score_a(d.to_owned()), 15691);
    }

    #[test]
    fn test_b() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day2/input.txt");

        assert_eq!(find_score_b(d.to_owned()), 12989);
    }
}
