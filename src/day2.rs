pub fn run(input: &str) -> String {
    part1(input)
}

fn part1(input: &str) -> String {
    let tmp = input.replace("\n", " ");
    let clean_input = tmp.split(" ");

    let mut directions: Vec<String> = Vec::new();
    let mut values: Vec<i32> = Vec::new();

    for (i, s) in clean_input.enumerate() {
        if i % 2 == 0 {
            directions.push(s.to_string());
        } else {
            values.push(s.parse::<i32>().unwrap());
        }
    }

    let mut x_pos = 0;
    let mut y_pos = 0;

    assert!(directions.len() == values.len());
    for (i, dir) in directions.iter().enumerate() {
        match dir.as_str() {
            "forward" => x_pos += values.iter().nth(i).unwrap(),
            "up" => y_pos -= values.iter().nth(i).unwrap(),
            "down" => y_pos += values.iter().nth(i).unwrap(),
            _ => panic!("Invalid direction"),
        }
    }

    format!("[day 2] Final location is {}", x_pos * y_pos)
}
