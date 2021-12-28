pub fn run(input: &str) -> String {
    let num_strs: Vec<&str> = input.split("\n").collect();

    let pc = part1(&num_strs);
    assert_eq!(4139586, pc);

    let l = part2(&num_strs);
    assert_eq!(1800151, l);

    format!(
        "power consumption is {}\n\tlife support rating is {}",
        pc, l
    )
}

fn part1(num_strs: &Vec<&str>) -> i32 {
    let num_cnt = num_strs.len();
    let bit_cnt = num_strs.first().unwrap().len();

    let mut high_cnts = vec![0; bit_cnt];

    for num in num_strs {
        for (index, c) in num.chars().enumerate() {
            match c {
                '1' => high_cnts[index] += 1,
                '0' => continue,
                _ => panic!("something"),
            }
        }
    }

    let mut gamma_rate: i32 = 0;
    for n in high_cnts {
        gamma_rate = gamma_rate << 1;
        if n > num_cnt / 2 {
            gamma_rate += 1;
        } else if n < num_cnt / 2 {
            continue;
        } else {
            panic!("this should not happen");
        }
    }

    let epsilon_mask = 2i32.pow(bit_cnt as u32) - 1;
    let epsilon_rate = !gamma_rate & epsilon_mask;

    let pwr_consumption = gamma_rate * epsilon_rate;

    return pwr_consumption;
}

fn part2(num_strs: &Vec<&str>) -> u32 {
    o2_gen_rating(num_strs) * co2_scrubber_rating(num_strs)
}

fn o2_gen_rating(num_strs: &Vec<&str>) -> u32 {
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

        mask >>= 1;
    }

    let o2 = nums.first().unwrap();

    return *o2;
}

fn co2_scrubber_rating(num_strs: &Vec<&str>) -> u32 {
    let mut nums: Vec<u32> = num_strs
        .iter()
        .map(|s| u32::from_str_radix(s, 2).expect("not a valid binary number"))
        .collect();

    let bit_cnt = num_strs.first().unwrap().len() as u32;
    let mut mask = 2u32.pow(bit_cnt - 1);

    while nums.len() > 1 {
        let high_cnts = nums.iter().filter(|&n| n & mask == mask).count();

        let mut filter = mask;
        if high_cnts * 2 >= nums.len() {
            filter = 0;
        }

        nums = nums.into_iter().filter(|&n| n & mask == filter).collect();

        mask >>= 1;
    }
    let co2 = *nums.first().unwrap();

    co2
}
