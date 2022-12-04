#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod a {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    use itertools::Itertools;

    pub fn find_completly_overlapping_sections(file_path: PathBuf) -> usize {
        BufReader::new(File::open(file_path).unwrap())
            .lines()
            .map(|line| line.unwrap().parse::<String>().unwrap())
            .collect::<Vec<String>>()
            .iter()
            .flat_map(|line| line.split(','))
            .flat_map(|pair| pair.split('-'))
            .map(|id| id.parse::<usize>().unwrap())
            .tuples()
            .fold(0, |acc, (id1, id2, id3, id4)| {
                if (id1 <= id3 && id2 >= id4) || (id3 <= id1 && id4 >= id2) {
                    acc + 1
                } else {
                    acc
                }
            })
    }
}

pub mod b {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    use itertools::Itertools;

    pub fn find_overlapping_sections(file_path: PathBuf) -> usize {
        BufReader::new(File::open(file_path).unwrap())
            .lines()
            .map(|line| line.unwrap().parse::<String>().unwrap())
            .collect::<Vec<String>>()
            .iter()
            .flat_map(|line| line.split(','))
            .flat_map(|pair| pair.split('-'))
            .map(|id| id.parse::<usize>().unwrap())
            .tuples()
            .fold(0, |acc, (id1, id2, id3, id4)| {
                if (id1 <= id3 && id3 <= id2) || (id3 <= id1 && id1 <= id4) {
                    acc + 1
                } else {
                    acc
                }
            })
    }
}

mod tests {
    use crate::day4::a::find_completly_overlapping_sections;
    use crate::day4::b::find_overlapping_sections;

    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day4/input_test.txt");

        assert_eq!(find_completly_overlapping_sections(d.to_owned()), 2);
    }

    #[test]
    fn test_a() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day4/input.txt");

        assert_eq!(find_completly_overlapping_sections(d.to_owned()), 644);
    }

    #[test]
    fn test_b() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day4/input.txt");

        assert_eq!(find_overlapping_sections(d.to_owned()), 926);
    }
}
