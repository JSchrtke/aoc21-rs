pub fn run(input: &str) -> String {
    part2(input)
}

// Since this is only used in part one, and can't easily be refactored to extract the duplicated
// code, the warning about dead code is ignored here.
#[allow(dead_code)]
fn part1(input: &str) -> String {
    let mut x = 0;
    let mut y = 0;

    input.split("\n").for_each(|s| {
        let mut iter = s.split(" ");
        let s = iter.next().unwrap();
        let n = iter.next().unwrap().parse::<i32>().unwrap();
        match s {
            "forward" => x += n,
            "up" => y -= n,
            "down" => y += n,
            _ => panic!("Invalid direction"),
        }
    });

    format!("[day 2] Final location is {}", x * y)
}

fn part2(input: &str) -> String {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    input.split("\n").for_each(|s| {
        let mut iter = s.split(" ");
        let s = iter.next().unwrap();
        let n = iter.next().unwrap().parse::<i32>().unwrap();
        match s {
            "down" => aim += n,
            "up" => aim -= n,
            "forward" => {
                x += n;
                y += n * aim;
            }
            _ => panic!("Invalid direction"),
        }
    });

    format!("Final location is {}", x * y)
}
