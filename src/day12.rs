pub fn answers(input: Vec<String>) -> (i32, i32) {
    // Parsing

    let mut plants: Vec<Vec<char>> = Vec::new();

    for line in input {
        plants.push(line.chars().collect());
    }

    let height = plants.len() as usize;
    let width = plants[0].len() as usize;

    // Answer 1

    let mut ans1: i32 = 0;

    // Answer 2

    let mut ans2: i32 = 0;

    let mut claimed: Vec<Vec<bool>> = vec![vec![false; width]; height];
    let mut regions: Vec<Vec<(i32, i32)>> = Vec::new();

    for i in 0..plants.len() {
        for j in 0..plants[i].len() {
            if !claimed[i][j] {
                let plant = plants[i][j];
                let mut open_list: Vec<(i32, i32)> = vec![(i as i32, j as i32)];
                let mut region_list: Vec<(i32, i32)> = Vec::new();

                while let Some((y, x)) = open_list.pop() {
                    region_list.push((y, x));
                    claimed[y as usize][x as usize] = true;

                    let mut push = |pos: (i32, i32)| {
                        if plants[pos.0 as usize][pos.1 as usize] == plant
                            && !region_list.contains(&pos)
                            && !open_list.contains(&pos)
                        {
                            open_list.push(pos);
                        }
                    };

                    if x > 0 {
                        push((y, x - 1))
                    }
                    if x < width as i32 - 1 {
                        push((y, x + 1))
                    }
                    if y > 0 {
                        push((y - 1, x))
                    }
                    if y < height as i32 - 1 {
                        push((y + 1, x))
                    }
                }

                regions.push(region_list);
            }
        }
    }

    for region in regions {
        let area = region.len() as i32;
        let mut perimeter = 0;
        let mut sides = 0;

        for &(i, j) in region.iter() {
            if !region.contains(&(i - 1, j)) {
                perimeter += 1;
            }
            if !region.contains(&(i + 1, j)) {
                perimeter += 1;
            }
            if !region.contains(&(i, j - 1)) {
                perimeter += 1;
            }
            if !region.contains(&(i, j + 1)) {
                perimeter += 1;
            }
        }

        for &(i, j) in region.iter() {
            let check = |delta_i: i32, delta_j: i32| region.contains(&(i + delta_i, j + delta_j));

            if !check(-1, -1) && !(check(0, -1) ^ check(-1, 0)) {
                sides += 1;
            }

            if !check(-1, 1) && !(check(0, 1) ^ check(-1, 0)) {
                sides += 1;
            }

            if !check(1, 1) && !(check(0, 1) ^ check(1, 0)) {
                sides += 1;
            }

            if !check(1, -1) && !(check(0, -1) ^ check(1, 0)) {
                sides += 1;
            }

            if check(-1, -1) && !(check(0, -1) || check(-1, 0)) {
                sides += 1;
            }

            if check(-1, 1) && !(check(0, 1) || check(-1, 0)) {
                sides += 1;
            }

            if check(1, 1) && !(check(0, 1) || check(1, 0)) {
                sides += 1;
            }

            if check(1, -1) && !(check(0, -1) || check(1, 0)) {
                sides += 1;
            }
        }

        ans1 += area * perimeter;
        ans2 += area * sides;
    }

    // Return

    (ans1, ans2)
}
