pub fn run(input: &str) -> String {
    part2(input)
}

fn part1(input: &str) -> String {
    let mut x_pos = 0;
    let mut y_pos = 0;

    input.split("\n").for_each(|s| {
        let mut it = s.split(" ");
        let s = it.next().unwrap();
        let n = it.next().unwrap().parse::<i32>().unwrap();
        match s {
            "forward" => x_pos += n,
            "up" => y_pos -= n,
            "down" => y_pos += n,
            _ => panic!("Invalid direction"),
        }
    });

    format!("[day 2] Final location is {}", x_pos * y_pos)
}

fn part2(input: &str) -> String {
    let mut x_pos = 0;
    let mut y_pos = 0;
    let mut aim = 0;

    input.split("\n").for_each(|s| {
        let mut it = s.split(" ");
        let s = it.next().unwrap();
        let n = it.next().unwrap().parse::<i32>().unwrap();
        match s {
            "down" => aim += n,
            "up" => aim -= n,
            "forward" => {
                x_pos += n;
                y_pos += n * aim;
            }
            _ => panic!("Invalid direction"),
        }
    });

    format!("Final location is {}", x_pos * y_pos)
}
