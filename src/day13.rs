#[derive(Clone)]
struct Problem {
    x: i64,
    y: i64,
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
}

fn get_a_b(p: &Problem) -> (i64, i64) {
    let det = p.a_x * p.b_y - p.a_y * p.b_x;
    let a = p.b_y * p.x - p.b_x * p.y;
    let b = -p.a_y * p.x + p.a_x * p.y;

    if a % det == 0 && b % det == 0 {
        (a / det, b / det)
    } else {
        (0, 0)
    }
}

pub fn answers(input: Vec<String>) -> (i64, i64) {
    // Parsing

    let mut line_index = 0;
    let mut problems: Vec<Problem> = Vec::new();
    let mut problem: Problem = Problem {
        x: 0,
        y: 0,
        a_x: 0,
        a_y: 0,
        b_x: 0,
        b_y: 0,
    };

    for line in input {
        if line_index <= 2 {
            let numbers: Vec<i64> = line
                .split(",")
                .map(|s| s.chars().filter(|c| c.is_digit(10)).collect::<String>())
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            if line_index == 0 {
                problem.a_x = numbers[0];
                problem.a_y = numbers[1];
            } else if line_index == 1 {
                problem.b_x = numbers[0];
                problem.b_y = numbers[1];
            } else if line_index == 2 {
                problem.x = numbers[0];
                problem.y = numbers[1];
                problems.push(problem.clone());
            }

            line_index += 1;
        } else {
            line_index = 0;
        }
    }

    // Answer 1

    let mut ans1 = 0;

    for p in problems.iter() {
        let (a, b) = get_a_b(&p);
        ans1 += a * 3 + b;
    }

    // Answer 2

    let mut ans2: i64 = 0;

    for p in problems.iter() {
        let mut big_p = p.clone();
        big_p.x += 10000000000000;
        big_p.y += 10000000000000;

        let (a, b) = get_a_b(&big_p);
        ans2 += a * 3 + b;
    }

    // Return

    (ans1, ans2)
}
