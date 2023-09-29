#[derive(Debug, PartialEq)]
enum EqualityResult {
    IndexOfFirstDuplicate(usize),
    NoDuplicates,
}

fn check_quartet_for_duplicates(s: &[u8]) -> Result<EqualityResult, ()> {
    if s.len() != 4 {
        Err(())
    } else if s[0] == s[1] || s[0] == s[2] || s[0] == s[3] {
        Ok(EqualityResult::IndexOfFirstDuplicate(0))
    } else if s[1] == s[2] || s[1] == s[3] {
        Ok(EqualityResult::IndexOfFirstDuplicate(1))
    } else if s[2] == s[3] {
        Ok(EqualityResult::IndexOfFirstDuplicate(2))
    } else {
        Ok(EqualityResult::NoDuplicates)
    }
}

fn find_first_nonduplicate_quartet_end_index(data: &[u8]) -> Result<usize, &'static str> {
    let mut i = 0;
    while i + 3 < data.len() {
        i = i + match check_quartet_for_duplicates(&data[i..i + 4]) {
            Ok(EqualityResult::NoDuplicates) => {
                return Ok(i + 4);
            }
            Ok(EqualityResult::IndexOfFirstDuplicate(ind)) => ind + 1,
            _ => {
                println!(
                    "Error when checking equality of elements, \n\tindices starting at [{}], \n",
                    i
                );
                println!(
                    "\telements [{}, {}, {}, {}]",
                    data[i] as char,
                    data[i + 1] as char,
                    data[i + 2] as char,
                    data[i + 3] as char,
                );
                1
            }
        }
    }

    Err("Reached end of string with no duplicates found")
}

fn main() {
    let data = include_str!("input.txt").as_bytes();
    match find_first_nonduplicate_quartet_end_index(data) {
        Ok(i) => {
            println!(
                "[{}, {}, {}, {}]",
                data[i - 4] as char,
                data[i - 3] as char,
                data[i - 2] as char,
                data[i - 1] as char,
            );
            println!("{}", i);
        }
        Err(s) => println!("Error: {}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test given on website
    #[test]
    fn test1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes();
        assert_eq!(find_first_nonduplicate_quartet_end_index(input), Ok(5));
    }

    #[test]
    fn test2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg".as_bytes();
        assert_eq!(find_first_nonduplicate_quartet_end_index(input), Ok(6));
    }

    #[test]
    fn test3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes();
        assert_eq!(find_first_nonduplicate_quartet_end_index(input), Ok(10));
    }

    #[test]
    fn test4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes();
        assert_eq!(find_first_nonduplicate_quartet_end_index(input), Ok(11));
    }

    // Not really needed, written for practice
    #[test]
    fn test_check_quartet_with_duplicates() {
        let input = "mjqj".as_bytes();
        assert_eq!(
            check_quartet_for_duplicates(input),
            Ok(EqualityResult::IndexOfFirstDuplicate(1))
        );
    }

    #[test]
    fn test_check_quartet_with_no_duplicates() {
        let input = "mjqw".as_bytes();
        assert_eq!(
            check_quartet_for_duplicates(input),
            Ok(EqualityResult::NoDuplicates)
        );
    }

    #[test]
    fn test_check_quartet_with_too_small_input() {
        let input = "mjq".as_bytes();
        assert_eq!(check_quartet_for_duplicates(input), Err(()));
    }

    #[test]
    fn test_check_quartet_with_too_large_input() {
        let input = "mjqwt".as_bytes();
        assert_eq!(check_quartet_for_duplicates(input), Err(()));
    }
}
