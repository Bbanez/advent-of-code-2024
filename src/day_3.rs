use crate::load_file::load_file;

pub fn day_3() {
    let corrupted_mem = load_file("day_3.txt").unwrap();
    // let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let mul_values = find_values(corrupted_mem.as_str());
    let mul_value: i32 = mul_values.iter().map(|(a, b)| a * b).sum();
    println!("Multiplication value: {}", mul_value);
    // let test_input_2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let mul_values_acc = find_values_acc(corrupted_mem.as_str());
    let mul_value_acc: i32 = mul_values_acc.iter().map(|(a, b)| a * b).sum();
    println!("Multiplication value accurate: {}", mul_value_acc);
}

fn find_values_acc(input: &str) -> Vec<(i32, i32)> {
    let mul_indexes = find_mul_indexes(input);
    let skip_mul_command = "don't()";
    let do_not_skip_mul_command = "do()";
    let mut output: Vec<(i32, i32)> = vec![];
    let mut skip = false;
    for i in 0..mul_indexes.len() {
        if i == 0 {
            let skip_mul_index = input[0..mul_indexes[i].0].find(skip_mul_command);
            if let Some(_) = skip_mul_index {
                skip = true;
            }
        } else {
            if skip {
                let skip_mul_index =
                    input[mul_indexes[i - 1].1..mul_indexes[i].0].find(do_not_skip_mul_command);
                if let Some(_) = skip_mul_index {
                    skip = false;
                }
            } else {
                let skip_mul_index =
                    input[mul_indexes[i - 1].1..mul_indexes[i].0].find(skip_mul_command);
                if let Some(_) = skip_mul_index {
                    skip = true;
                }
            }
        }
        if skip {
            continue;
        }
        let sub_str = &input[mul_indexes[i].0..mul_indexes[i].1];
        let parts: Vec<&str> = sub_str.split(",").collect();
        if parts.len() != 2 {
            continue;
        }
        output.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()))
    }
    output
}

fn find_values(input: &str) -> Vec<(i32, i32)> {
    let mul_indexes = find_mul_indexes(input);
    let mut output: Vec<(i32, i32)> = vec![];
    for i in 0..mul_indexes.len() {
        let sub_str = &input[mul_indexes[i].0..mul_indexes[i].1];
        let parts: Vec<&str> = sub_str.split(",").collect();
        if parts.len() != 2 {
            continue;
        }
        output.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()))
    }
    output
}

fn find_mul_indexes(input: &str) -> Vec<(usize, usize)> {
    let start = "mul(";
    let end = ")";
    let mut mul_indexes: Vec<(usize, usize)> = vec![];
    let mut at_index: usize = 0;
    let max_size = 7;
    while at_index < input.len() {
        let start_index = input[at_index..input.len()].find(start);
        if let Some(a) = start_index {
            let start_idx = a + at_index + start.len();
            let end_index = input[start_idx..input.len()].find(end);
            if let Some(b) = end_index {
                let end_idx = b + start_idx;
                if end_idx - start_idx > max_size {
                    at_index += 1;
                    continue;
                }
                mul_indexes.push((start_idx, end_idx));
                at_index = end_idx - 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    mul_indexes
}
