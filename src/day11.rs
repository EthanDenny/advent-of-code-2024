use std::collections::HashMap;

fn descend(n: i64) -> Vec<i64> {
    let s = n.to_string();

    if n == 0 {
        vec![1]
    } else if s.len() % 2 == 0 {
        vec![
            String::from(&s[0..s.len() / 2]).parse::<i64>().unwrap(),
            String::from(&s[s.len() / 2..s.len()])
                .parse::<i64>()
                .unwrap(),
        ]
    } else {
        vec![n * 2024]
    }
}

fn get_size_after_iters(
    numbers: &Vec<i64>,
    descents: &mut HashMap<i64, Vec<i64>>,
    depth: i64,
) -> i64 {
    if depth == 0 {
        numbers.len() as i64
    } else {
        numbers
            .iter()
            .map(|&n| {
                if !descents.contains_key(&n) {
                    descents.insert(n, descend(n));
                }

                get_size_after_iters(&descents[&n].clone(), descents, depth - 1)
            })
            .sum()
    }
}

pub fn answers(input: Vec<String>) -> (i64, i64) {
    // Parsing

    let mut numbers: Vec<i64> = Vec::new();

    for line in input {
        for number in line.split(" ") {
            numbers.push(number.parse::<i64>().unwrap());
        }
    }

    let mut descents: HashMap<i64, Vec<i64>> = HashMap::new();

    // Answer 1

    let ans1 = get_size_after_iters(&numbers, &mut descents, 25);

    // Answer 2

    let ans2 = get_size_after_iters(&numbers, &mut descents, 75);

    // Return

    (ans1, ans2)
}
