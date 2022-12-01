#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod a {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    pub fn find_elf_carrying_most_calories(file_path: PathBuf) -> usize {
        BufReader::new(File::open(file_path).unwrap())
            .lines()
            .map(|line| line.unwrap().parse::<String>().unwrap())
            .collect::<Vec<String>>()
            .split(|e| e.is_empty())
            .map(|x| x.iter().map(|e| e.parse::<usize>().unwrap()).sum::<usize>())
            .max()
            .unwrap()
    }
}

pub mod b {
    use std::collections::BTreeSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    pub fn find_sum_of_top_3(file_path: PathBuf) -> usize {
        BufReader::new(File::open(file_path).unwrap())
            .lines()
            .map(|line| line.unwrap().parse::<String>().unwrap())
            .collect::<Vec<String>>()
            .split(|e| e.is_empty())
            .map(|x| x.iter().map(|e| e.parse::<usize>().unwrap()).sum::<usize>())
            .collect::<BTreeSet<usize>>()
            .iter()
            .rev()
            .take(3)
            .sum::<usize>()
    }
}

mod tests {
    use crate::day1::a::find_elf_carrying_most_calories;
    use crate::day1::b::find_sum_of_top_3;

    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day1/input_test.txt");

        assert_eq!(find_elf_carrying_most_calories(d.to_owned()), 24000);
    }

    #[test]
    fn test_a() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day1/input.txt");

        assert_eq!(find_elf_carrying_most_calories(d.to_owned()), 75622);
    }

    #[test]
    fn test_b() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day1/input.txt");

        assert_eq!(find_sum_of_top_3(d.to_owned()), 213159);
    }
}
