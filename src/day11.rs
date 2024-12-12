pub fn answers(input: Vec<String>) -> (i32, i32) {
    // Parsing

    let mut numbers = Vec::new();

    for line in input {
        for number in line.split(" ") {
            numbers.push(String::from(number));
        }
    }

    let blink = |numbers: Vec<String>| {
        let mut new_numbers = Vec::new();

        for n in numbers.iter() {
            if n == "0" {
                new_numbers.push(String::from("1"));
            } else if n.len() % 2 == 0 {
                new_numbers.push(
                    String::from(&n[0..n.len() / 2])
                        .parse::<i64>()
                        .unwrap()
                        .to_string(),
                );
                new_numbers.push(
                    String::from(&n[n.len() / 2..n.len()])
                        .parse::<i64>()
                        .unwrap()
                        .to_string(),
                );
            } else {
                new_numbers.push((n.parse::<i64>().unwrap() * 2024).to_string())
            }
        }

        return new_numbers;
    };

    // Answer 1

    let mut ans1 = 0;

    for _ in 0..25 {
        numbers = blink(numbers);
    }

    ans1 = numbers.len() as i32;

    // Answer 2

    let mut ans2 = 0;

    // Return

    (ans1, ans2)
}
