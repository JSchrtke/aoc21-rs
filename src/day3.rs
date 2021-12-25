pub fn run(input: &str) -> String {
    format!("power consumption is {}", day3(input))
}

fn day3(input: &str) -> i32 {
    let num_list: Vec<&str> = input.split("\n").collect();
    let num_cnt = num_list.len();
    let num_len = num_list.first().unwrap().len();

    let mut counts = vec![0; num_len];

    for num in num_list {
        for (index, c) in num.chars().enumerate() {
            match c {
                '1' => counts[index] += 1,
                '0' => continue,
                _ => panic!("something"),
            }
        }
    }

    let mut gamma_rate: i32 = 0;
    for n in counts {
        gamma_rate = gamma_rate << 1;
        if n > num_cnt / 2 {
            gamma_rate += 1;
        } else if n < num_cnt / 2 {
            continue;
        } else {
            panic!("this should not happen");
        }
    }

    let bit_cnt = num_len as u32;
    const BASE: i32 = 2;
    let epsilon_mask = BASE.pow(bit_cnt) - 1;
    let epsilon_rate = !gamma_rate & epsilon_mask;

    gamma_rate * epsilon_rate
}
