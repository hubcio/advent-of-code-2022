#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod a {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[derive(Debug)]
    struct Move {
        n: u32,
        from: u32,
        to: u32,
    }

    impl FromStr for Move {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let words: Vec<String> = s.split_whitespace().map(|word| word.to_string()).collect();

            let from: u32 = words[3].parse().unwrap();
            let to: u32 = words[5].parse().unwrap();
            let n = words[1].parse().unwrap();

            Ok(Move {
                n,
                from: from - 1,
                to: to - 1,
            })
        }
    }

    pub fn find_top_crates(file_path: PathBuf) -> String {
        let mut iter = BufReader::new(File::open(file_path).unwrap())
            .lines()
            .map(|line| line.unwrap());

        const VAL: Vec<u8> = Vec::new();
        let mut stacks = [VAL; 9];

        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }

            let line = line.as_bytes();
            for i in (1..line.len()).step_by(4) {
                if line[i] == b'1' {
                    continue;
                }
                if line[i] != b' ' {
                    stacks[i / 4].insert(0, line[i]);
                }
            }
        }

        iter.map(|line| line.parse::<Move>().unwrap())
            .for_each(|m| {
                for _i in 0..m.n {
                    let c = stacks[m.from as usize].pop().unwrap();
                    stacks[m.to as usize].push(c);
                }
            });

        stacks.iter().fold(String::new(), |mut acc, stack| {
            if let Some(&c) = stack.last() {
                acc.push(c as char);
            }
            acc
        })
    }
}
pub mod b {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;
    use std::str::FromStr;

    #[derive(Debug)]
    struct Move {
        n: u32,
        from: u32,
        to: u32,
    }

    impl FromStr for Move {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let words: Vec<String> = s.split_whitespace().map(|word| word.to_string()).collect();

            let from: u32 = words[3].parse().unwrap();
            let to: u32 = words[5].parse().unwrap();
            let n = words[1].parse().unwrap();

            Ok(Move {
                n,
                from: from - 1,
                to: to - 1,
            })
        }
    }

    pub fn find_top_crates_b(file_path: PathBuf) -> String {
        let mut iter = BufReader::new(File::open(file_path).unwrap())
            .lines()
            .map(|line| line.unwrap());

        const VAL: Vec<u8> = Vec::new();
        let mut stacks = [VAL; 9];

        for line in iter.by_ref() {
            if line.is_empty() {
                break;
            }

            let line = line.as_bytes();
            for i in (1..line.len()).step_by(4) {
                if line[i] == b'1' {
                    continue;
                }
                if line[i] != b' ' {
                    stacks[i / 4].insert(0, line[i]);
                }
            }
        }

        iter.map(|line| line.parse::<Move>().unwrap())
            .for_each(|m| {
                let idx = stacks[m.from as usize].len() - m.n as usize;
                for _i in 0..m.n {
                    let c = stacks[m.from as usize][idx];
                    stacks[m.from as usize].remove(idx);
                    stacks[m.to as usize].push(c);
                }
            });

        stacks.iter().fold(String::new(), |mut acc, stack| {
            if let Some(&c) = stack.last() {
                acc.push(c as char);
            }
            acc
        })
    }
}
mod tests {
    use crate::day5::a::find_top_crates;
    use crate::day5::b::find_top_crates_b;

    use std::path::PathBuf;

    #[test]
    fn test_a() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day5/input.txt");

        assert_eq!(find_top_crates(d.to_owned()), "CNSZFDVLJ");
    }

    #[test]
    fn test_b() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day5/input.txt");

        assert_eq!(find_top_crates_b(d.to_owned()), "QNDWLMGNS");
    }
}
