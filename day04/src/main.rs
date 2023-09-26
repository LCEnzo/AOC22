use std::num::ParseIntError;

fn main() {
    let input_txt = include_str!("input.txt");
    let count = input_txt
        .lines()
        .filter_map(|line| {
            let pairs_of_section_bounds: Vec<_> = line.split(',')
                .take(2)
                .filter_map(|section| str_to_sections_bounds(section).ok())
                .collect();

            if pairs_of_section_bounds.len() == 2 {
                Some(pairs_of_section_bounds)
            } else {
                None
            }
        })
        .filter(|pair_of_bounds| {
            let (start1, end1) = pair_of_bounds[0];
            let (start2, end2) = pair_of_bounds[1];

            let first_is_inside_second = start1 >= start2 && end1 <= end2;
            let second_is_inside_first = start1 <= start2 && end1 >= end2;
            
            first_is_inside_second || second_is_inside_first
        })
        .count();

    println!("{}", count);
}

fn str_to_sections_bounds(section: &str) -> Result<(u32, u32), ParseIntError> {
    let mut bounds = section
        .split('-')
        .take(2)
        .map(|x| x.parse::<u32>());
    
    let start = bounds.next().unwrap_or(Ok(0))?;
    let end = bounds.next().unwrap_or(Ok(0))?;

    Ok((start, end))
}
