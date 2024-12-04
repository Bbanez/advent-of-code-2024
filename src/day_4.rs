use crate::load_file::load_file;

const XMAS_BYTES: [u8; 4] = [88, 77, 65, 83];
const X_MAS_BYTES: [u8; 3] = [77, 65, 83];

pub fn day_4() {
    let search_input = load_file("day_4.txt").unwrap();
    let test_input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    // let search_bytes = test_input.as_bytes();
    let search_bytes = search_input.as_bytes();
    let mut lock_width = false;
    let mut width: usize = 0;
    let mut height: usize = 1;
    for i in 0..search_bytes.len() {
        if lock_width == false {
            width += 1;
        }
        if search_bytes[i] == 10 {
            height += 1;
            if !lock_width {
                width -= 1;
                lock_width = true;
            }
        }
    }
    println!(
        "XMAS count: {}",
        find_xmas_count(search_bytes, width, height)
    );
    println!(
        "X_MAS count: {}",
        find_x_mas_count(search_bytes, width, height)
    );
}

fn find_x_mas_count(bytes: &[u8], width: usize, height: usize) -> usize {
    let mut count: usize = 0;
    let mut idx = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            idx = xy_to_idx(width + 1, x, y);
            if bytes[idx] != X_MAS_BYTES[1] {
                continue;
            }
            let top_left = bytes[xy_to_idx(width + 1, x - 1, y - 1)];
            let top_right = bytes[xy_to_idx(width + 1, x + 1, y - 1)];
            let bottom_right = bytes[xy_to_idx(width + 1, x + 1, y + 1)];
            let bottom_left = bytes[xy_to_idx(width + 1, x - 1, y + 1)];
            if ((top_left == X_MAS_BYTES[0] && bottom_right == X_MAS_BYTES[2])
                || (top_left == X_MAS_BYTES[2] && bottom_right == X_MAS_BYTES[0]))
                && ((bottom_left == X_MAS_BYTES[0] && top_right == X_MAS_BYTES[2])
                    || (bottom_left == X_MAS_BYTES[2] && top_right == X_MAS_BYTES[0]))
            {
                count += 1;
            }
        }
    }
    count
}

fn find_xmas_count(bytes: &[u8], width: usize, height: usize) -> usize {
    let mut count: usize = 0;
    let mut idx = 0;
    for y in 0..height {
        for x in 0..width {
            idx = xy_to_idx(width + 1, x, y);
            if idx >= bytes.len() {
                break;
            }
            if xmas_horizontal_rule(bytes, idx) {
                count += 1;
            }
            if xmas_vertical_rule(bytes, width, x, y, idx) {
                count += 1;
            }
            count += xmas_diagonal_rule(bytes, width, height, x, y, idx);
        }
    }
    count
}

fn xmas_diagonal_rule(
    bytes: &[u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    idx: usize,
) -> usize {
    let xmas_offset;
    if bytes[idx] == XMAS_BYTES[3] {
        xmas_offset = 3;
    } else if bytes[idx] == XMAS_BYTES[0] {
        xmas_offset = 0;
    } else {
        return 0;
    }
    let mut found_left = true;
    let mut found_right = true;
    for i in 1..4 {
        if x + i > width || y + i > height {
            found_right = false;
        } else if (x as i32) - (i as i32) < 0 || y + i > height {
            found_left = false
        }
        let xmas_idx;
        if xmas_offset > 0 {
            xmas_idx = 3 - i;
        } else {
            xmas_idx = i
        }
        if found_right {
            let right_idx = xy_to_idx(width + 1, x + i, y + i);
            if right_idx >= bytes.len() || bytes[right_idx] != XMAS_BYTES[xmas_idx] {
                found_right = false;
            }
        }
        if found_left {
            let left_idx = xy_to_idx(width + 1, x - i, y + i);
            if left_idx >= bytes.len() || bytes[left_idx] != XMAS_BYTES[xmas_idx] {
                found_left = false;
            }
        }
        if !found_right && !found_left {
            break;
        }
    }
    if found_right && found_left {
        return 2;
    } else if found_right || found_left {
        return 1;
    }
    0
}

fn xmas_vertical_rule(bytes: &[u8], width: usize, x: usize, y: usize, idx: usize) -> bool {
    let xmas_offset;
    if bytes[idx] == XMAS_BYTES[3] {
        xmas_offset = 3;
    } else if bytes[idx] == XMAS_BYTES[0] {
        xmas_offset = 0;
    } else {
        return false;
    }
    let mut found = true;
    for i in 1..4 {
        let next_idx = xy_to_idx(width + 1, x, y + i);
        let xmas_idx;
        if xmas_offset > 0 {
            xmas_idx = 3 - i;
        } else {
            xmas_idx = i
        }
        if next_idx >= bytes.len() {
            found = false;
            break;
        }
        if bytes[next_idx] != XMAS_BYTES[xmas_idx] {
            found = false;
            break;
        }
    }
    found
}

fn xmas_horizontal_rule(bytes: &[u8], idx: usize) -> bool {
    let xmas_offset;
    if bytes[idx] == XMAS_BYTES[3] {
        xmas_offset = 3;
    } else if bytes[idx] == XMAS_BYTES[0] {
        xmas_offset = 0;
    } else {
        return false;
    }
    let mut found = true;
    for i in 1..4 {
        let next_idx = idx + i;
        if next_idx >= bytes.len() {
            found = false;
            break;
        }
        let xmas_idx;
        if xmas_offset > 0 {
            xmas_idx = 3 - i;
        } else {
            xmas_idx = i
        }
        if bytes[next_idx] != XMAS_BYTES[xmas_idx] {
            found = false;
            break;
        }
    }
    found
}

fn xy_to_idx(width: usize, x: usize, y: usize) -> usize {
    x + width * y
}

fn xy_to_idx_i32(width: i32, x: i32, y: i32) -> i32 {
    x + width * y
}

fn idx_to_xy(width: usize, idx: usize) -> (usize, usize) {
    let x = idx % width;
    let y = (idx - x) / width;
    (x, y)
}
