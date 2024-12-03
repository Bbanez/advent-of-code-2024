use crate::load_file::load_file;

pub fn day_2() {
    let file_content = load_file("day_2.txt").unwrap();
    let level_lines: Vec<&str> = file_content.split("\n").collect();
    let mut reports: Vec<Vec<i32>> = vec![];
    for i in 0..level_lines.len() {
        if level_lines[i] == "" {
            continue;
        }
        let levels_str: Vec<&str> = level_lines[i].split(" ").collect();
        let levels: Vec<i32> = levels_str.iter().map(|e| e.parse().unwrap()).collect();
        reports.push(levels);
    }
    println!("Safe reports count: {}", get_safe_reports_count(&reports));
    println!(
        "Safe reports count with dampener: {}",
        get_safe_reports_count_with_dampener(&reports)
    );
}

fn check_levels(levels: &Vec<i32>) -> bool {
    let max_delta = 3;
    let direction = levels[1] - levels[0];
    if direction == 0 || direction.abs() > max_delta {
        return false;
    }
    for i in 1..levels.len() - 1 {
        let delta = levels[i + 1] - levels[i];
        if delta == 0
            || delta.abs() > max_delta
            || direction > 0 && delta < 0
            || direction < 0 && delta > 0
        {
            return false;
        }
    }
    true
}

fn get_safe_reports_count(reports: &Vec<Vec<i32>>) -> usize {
    let mut count: usize = 0;
    for i in 0..reports.len() {
        if check_levels(&reports[i]) {
            count += 1;
        }
    }
    count
}

fn get_safe_reports_count_with_dampener(reports: &Vec<Vec<i32>>) -> usize {
    let mut count: usize = 0;
    for i in 0..reports.len() {
        let mut is_safe = false;
        let mut levels = reports[i].clone();
        for j in 0..levels.len()+1 {
            if check_levels(&levels) {
                is_safe = true;
                break;
            }
            if j < reports[i].len() {
                levels = reports[i].clone();
                levels.remove(j);
            }
        }
        if is_safe {
            count += 1;
            continue;
        }
    }
    count
}
