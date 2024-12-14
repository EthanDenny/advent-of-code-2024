struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

pub fn answers(input: Vec<String>) -> (i64, i64) {
    // Parsing

    let mut robots: Vec<Robot> = Vec::new();

    for line in input {
        let numbers: Vec<i64> = line
            .split(&[',', ' '][..])
            .map(|s| {
                s.chars()
                    .filter(|c| c.is_digit(10) || c == &'-')
                    .collect::<String>()
            })
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        robots.push(Robot {
            p: (numbers[0], numbers[1]),
            v: (numbers[2], numbers[3]),
        });
    }

    // Answer 1

    let mut ans1 = 0;

    {
        let width = 101;
        let height = 103;
        let seconds = 100;

        let mut quads = vec![0; 4];

        for robot in robots {
            let final_pos = (
                ((robot.p.0 + robot.v.0 * seconds) % width + width) % width,
                ((robot.p.1 + robot.v.1 * seconds) % height + height) % height,
            );

            let quad_x = {
                if final_pos.0 < width / 2 {
                    0
                } else if width / 2 < final_pos.0 && final_pos.0 < width {
                    2
                } else {
                    continue;
                }
            };

            if final_pos.1 < height / 2 {
                quads[quad_x] += 1;
            } else if height / 2 < final_pos.1 && final_pos.1 < height {
                quads[quad_x + 1] += 1;
            }
        }

        ans1 = quads.iter().product();
    }

    // Answer 2

    let mut ans2 = 0;

    // Return

    (ans1, ans2)
}
