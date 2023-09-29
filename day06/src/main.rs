#[derive(Debug, PartialEq)]
enum DuplicateIndex {
    Index(usize),
    NoDuplicates,
}

fn find_first_duplicate(s: &[u8]) -> DuplicateIndex {
    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            if s[i] == s[j] {
                return DuplicateIndex::Index(i);
            }
        }
    }

    DuplicateIndex::NoDuplicates
}

fn find_end_of_first_unique_substr(data: &[u8], len: usize) -> Result<usize, &'static str> {
    let mut i = 0;
    while i + len - 1 < data.len() {
        i = i + match find_first_duplicate(&data[i..i + len]) {
            DuplicateIndex::NoDuplicates => {
                return Ok(i + len);
            }
            DuplicateIndex::Index(ind) => ind + 1,
        }
    }

    Err("Reached end of data with no window of non duplicates found")
}

fn main() {
    let data = include_str!("input.txt").as_bytes();

    match find_end_of_first_unique_substr(data, 4) {
        Ok(i) => println!("First half: {}", i),
        Err(s) => println!("Error: {}", s),
    }

    match find_end_of_first_unique_substr(data, 14) {
        Ok(i) => println!("Second half: {}", i),
        Err(s) => println!("Error: {}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test21() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes();
        assert_eq!(find_end_of_first_unique_substr(input, 14), Ok(19));
    }

    #[test]
    fn test22() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes();
        assert_eq!(find_end_of_first_unique_substr(input, 14), Ok(23));
    }

    #[test]
    fn test23() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg".as_bytes();
        assert_eq!(find_end_of_first_unique_substr(input, 14), Ok(23));
    }

    #[test]
    fn test24() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes();
        assert_eq!(find_end_of_first_unique_substr(input, 14), Ok(29));
    }

    #[test]
    fn test25() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes();
        assert_eq!(find_end_of_first_unique_substr(input, 14), Ok(26));
    }

    // Not really needed, written for practice
    #[test]
    fn test_check_eq_with_duplicates() {
        let input = "mjqj".as_bytes();
        assert_eq!(find_first_duplicate(input), DuplicateIndex::Index(1));
    }

    #[test]
    fn test_check_eq_with_no_duplicates() {
        let input = "mjqw".as_bytes();
        assert_eq!(find_first_duplicate(input), DuplicateIndex::NoDuplicates);
    }

    #[test]
    fn test_check_eq_with_small_input() {
        let input = "m".as_bytes();
        assert_eq!(find_first_duplicate(input), DuplicateIndex::NoDuplicates);
    }

    #[test]
    fn test_check_eq_with_large_input() {
        let input = "mjqwtxyzp".as_bytes();
        assert_eq!(find_first_duplicate(input), DuplicateIndex::NoDuplicates);
    }
}
