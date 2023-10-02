use itertools::Itertools;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Folder {
    name: String,
    files_size: u128, // size of files in dir
    total_size: u128, // total size including subrdirs, sub sub folders, ...
    children: Vec<usize>,
    parent: usize
}

fn print_folder_tree(folders: &[Folder], index: usize, depth: u32) {
    let folder = &folders[index];
    for _ in 0..depth {
        print!("----");
    } print!("  ");
    
    println!("{} {} {}", folder.name, folder.files_size, folder.total_size);

    for &child_index in &folder.children {
        print_folder_tree(folders, child_index, depth + 1);
    }
}

fn parse_terminal_output(output: &str) -> Option<Vec<Folder>> {
    let mut folders = vec![Folder {
        name: "/".to_string(),
        files_size: 0,
        total_size: 0,
        children: vec![],
        parent: 0,
    }];
    let mut curr_index = 0; // root index

    for line in output.lines() {
        if line.len() < 4 {
            return None;
        }

        match line.split(" ").collect_vec()[..] {
            ["$", "ls"] => { /* Ignore */ },
            ["$", "cd", "/"] => { curr_index = 0; },
            ["dir", _] => { /* Ignore, since we already have the info via cd commmands */},
            ["$", "cd", ".."] => {
                let curr_folder = &folders[curr_index];
                curr_index = curr_folder.parent;
            },
            ["$", "cd", sub_path] => {
                let new_folder = Folder {
                    name: sub_path.to_string(),
                    files_size: 0,
                    total_size: 0,
                    children: vec![],
                    parent: curr_index,
                };
                folders.push(new_folder);
                let new_index = folders.len() - 1;
                folders[curr_index].children.push(new_index);
                curr_index = new_index;
            },
            [size_str, _] => {
                if let Ok(size) = size_str.parse::<u128>() {
                    folders[curr_index].files_size += size;
                }
            },
            _ => {}
        }
    }

    calc_folder_total_size(&mut folders, 0);
    // print_folder_tree(&folders, 0, 0);
    Some(folders)
}

fn calc_folder_total_size(folders: &mut [Folder], index: usize) {
    if folders[index].total_size != 0 {
        return;
    }

    let children_indices: Vec<usize> = folders[index].children.clone();

    for &child_index in &children_indices {
        calc_folder_total_size(folders, child_index);
    }

    folders[index].total_size = folders[index].files_size;

    for &child_index in &children_indices {
        folders[index].total_size += folders[child_index].total_size;
    }
}

fn find_sum_of_small_folder_sizes(folders: &[Folder], index: usize, max_size: u128) -> u128 {
    let folder = &folders[index];
    let own_contribution = if folder.total_size < max_size { folder.total_size } else { 0 };
    let child_contributions: u128 = folder
        .children
        .iter()
        .map(|&child_index| find_sum_of_small_folder_sizes(folders, child_index, max_size))
        .sum();
    
    own_contribution + child_contributions
}

fn find_min_folder_to_delete(folders: &[Folder]) -> Option<u128> {
    let target = 30000000 - (70000000 as u128 - folders[0].total_size);
    folders.iter()
        .map(|folder| folder.total_size)
        .filter(|size| size >= &target)
        .min()
}

fn main() {
    let input_txt = include_str!("input.txt");
    let folders = parse_terminal_output(input_txt).expect("Could not parse input text");
    println!("{}", find_sum_of_small_folder_sizes(&folders, 0, 100000));
    println!("{}", find_min_folder_to_delete(&folders).expect("Could not find a folder of size at least 30000000"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = include_str!("test_input1.txt");
        let folders = parse_terminal_output(input);

        assert_ne!(folders, None);

        let folders = folders.unwrap();

        assert!(itertools::any(folders.clone(), |folder| folder.name == "e" && folder.files_size == 584 as u128));
        assert!(itertools::any(folders.clone(), |folder| folder.name == "a" && folder.files_size == (94853 - 584) as u128));
        assert!(itertools::any(folders.clone(), |folder| folder.name == "d" && folder.files_size == 24933642 as u128));
        assert!(itertools::any(folders.clone(), |folder| folder.name == "/" && folder.files_size == (48381165 - 24933642 - 94853) as u128));
    }

    #[test]
    fn test_folder_total_size_calc() {
        let input = include_str!("test_input1.txt");
        let folders = parse_terminal_output(input);

        assert_ne!(folders, None);

        let folders = folders.unwrap();

        assert!(itertools::any(folders.clone(), |folder| folder.name == "e" && folder.total_size == 584 as u128));
        assert!(itertools::any(folders.clone(), |folder| folder.name == "a" && folder.total_size == 94853));
        assert!(itertools::any(folders.clone(), |folder| folder.name == "d" && folder.total_size == 24933642 as u128));
        assert!(itertools::any(folders.clone(), |folder| folder.name == "/" && folder.total_size == 48381165 as u128));
    }

    #[test]
    fn test_find_folder_for_deletion() {
        let input = include_str!("test_input1.txt");
        let folders = parse_terminal_output(input);

        assert_ne!(folders, None);

        let folders = folders.unwrap();
        let min = find_min_folder_to_delete(&folders).expect("Could not find a folder of size at least 30000000");

        assert_eq!(24933642, min);
    }

    #[test]
    fn test_first_half() {
        let input = include_str!("test_input1.txt");
        let folder = parse_terminal_output(input);
        
        if let Some(folders) = folder {
            assert_eq!(find_sum_of_small_folder_sizes(&folders, 0, 100000), 95437);
        } else {
            assert!(false);
        }
    }
}
