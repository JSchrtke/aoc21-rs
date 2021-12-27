pub fn run(input: &str) -> String {
    let pc = part1(input);
    let x = part2(input);
    format!("power consumption is {}\n\t{}", pc, x)
}

fn part1(input: &str) -> i32 {
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

    let epsilon_mask = 2i32.pow(num_len as u32) - 1;
    let epsilon_rate = !gamma_rate & epsilon_mask;

    gamma_rate * epsilon_rate
}

fn part2(input: &str) -> String {
    let n = f(input);
    format!("{:?}", n)
}

fn f(input: &str) -> &str {
    let mut num_strs: Vec<&str> = input.split("\n").collect();
    let bit_count = num_strs.first().unwrap().len();

    for bit_idx in 0..bit_count {
        if num_strs.len() == 1 {
            break;
        }

        let mut most_common_bit: char = '0';
        let mut high_count = 0;
        for s in &num_strs {
            let x = s;
            let y = x.chars().nth(bit_idx).unwrap();
            if y.eq(&'1') {
                high_count += 1
            };
            if high_count * 2 >= num_strs.len() {
                most_common_bit = '1';
                break;
            }
        }
        num_strs = num_strs
            .into_iter()
            .filter(|s| s.chars().nth(bit_idx).unwrap().eq(&most_common_bit))
            .collect();
    }

    num_strs.first().unwrap()
}
