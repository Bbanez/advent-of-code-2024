use crate::load_file::load_file;

pub fn day_1() {
    let file_content = load_file("day_1.txt").unwrap();
    let file_lines: Vec<&str> = file_content.split("\n").collect();
    let mut list_a: Vec<usize> = vec![];
    let mut list_b: Vec<usize> = vec![];
    for i in 0..file_lines.len() {
        let parts: Vec<&str> = file_lines[i].split("   ").collect();
        if parts[0] == "" || parts[1] == "" {
            continue;
        }
        list_a.push(parts[0].parse().unwrap());
        list_b.push(parts[1].parse().unwrap());
    }
    let distance = get_location_distances(&mut list_a.clone(), &mut list_b.clone());
    match distance {
        Ok(dist) => {
            println!("Total distance: {}", dist)
        }
        Err(err) => {
            panic!("{}", err)
        }
    }
    println!(
        "Total similarity score: {}",
        get_similarity_score(list_a, list_b)
    );
}

fn get_location_distances(
    list_a: &mut Vec<usize>,
    list_b: &mut Vec<usize>,
) -> Result<usize, String> {
    if list_a.len() != list_b.len() {
        return Err("List lengths must be same".to_string());
    }
    let mut total_distance: usize = 0;
    while list_a.len() > 0 {
        let mut smallest_a: usize = list_a[0];
        let mut a_idx = 0;
        let mut smallest_b = list_b[0];
        let mut b_idx = 0;
        for i in 1..list_a.len() {
            if list_a[i] < smallest_a {
                smallest_a = list_a[i];
                a_idx = i;
            }
        }
        for i in 1..list_b.len() {
            if list_b[i] < smallest_b {
                smallest_b = list_b[i];
                b_idx = i;
            }
        }
        list_a.remove(a_idx);
        list_b.remove(b_idx);
        if smallest_a < smallest_b {
            total_distance += smallest_b - smallest_a;
        } else {
            total_distance += smallest_a - smallest_b;
        }
    }
    Ok(total_distance)
}

fn get_similarity_score(list_a: Vec<usize>, list_b: Vec<usize>) -> usize {
    let mut sim_total: usize = 0;
    for i in 0..list_a.len() {
        let mut sub_sim = 0;
        for j in 0..list_b.len() {
            if list_a[i] == list_b[j] {
                sub_sim += 1;
            }
        }
        sim_total += list_a[i] * sub_sim;
    }
    sim_total
}
