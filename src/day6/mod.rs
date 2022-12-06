#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod a {
    use std::collections::HashSet;

    pub fn find_marker_index(input: &str, window_size: usize) -> usize {
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(window_size)
            .enumerate()
            .find(|(_, window)| window.iter().collect::<HashSet<&char>>().len() == window_size)
            .map(|(i, _)| i + window_size)
            .unwrap_or(0)
    }
}

mod tests {
    use crate::day6::a::find_marker_index;

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        assert_eq!(find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(
            find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
            10
        );
        assert_eq!(find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);

        assert_eq!(find_marker_index("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(find_marker_index("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(find_marker_index("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(
            find_marker_index("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
            29
        );
        assert_eq!(
            find_marker_index("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
            26
        );
    }

    #[test]
    fn test_b() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day6/input.txt");

        let input: String = BufReader::new(File::open(d).unwrap())
            .lines()
            .map(|line| line.unwrap())
            .collect();

        assert_eq!(find_marker_index(&input, 14), 3837);
    }
}
