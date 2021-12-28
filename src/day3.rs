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
    let o2 = o2_gen_rating(input);
    format!("o2 rating is {}", o2)
}

fn o2_gen_rating(input: &str) -> u32 {
    let num_strs: Vec<&str> = input.split("\n").collect();
    let bit_cnt = num_strs.first().unwrap().len();

    let mut nums: Vec<u32> = num_strs
        .iter()
        .map(|s| u32::from_str_radix(s, 2).unwrap())
        .collect();

    let mut mask = 2u32.pow(bit_cnt as u32 - 1);

    while nums.len() > 1 {
        let high_cnts = nums.iter().filter(|&n| n & mask == mask).count();
        let mut most_common = 0;
        if high_cnts * 2 >= nums.len() {
            most_common = 1;
        }

        let mut filter = 0;
        if most_common == 1 {
            filter = mask
        }

        nums = nums.into_iter().filter(|n| (n & mask) == filter).collect();

        mask = mask >> 1;
    }

    let res = nums.first().unwrap();

    *res
}
