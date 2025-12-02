const INPUT: &str = "9595822750-9596086139,1957-2424,88663-137581,48152-65638,12354817-12385558,435647-489419,518494-609540,2459-3699,646671-688518,195-245,295420-352048,346-514,8686839668-8686892985,51798991-51835611,8766267-8977105,2-17,967351-995831,6184891-6331321,6161577722-6161678622,912862710-913019953,6550936-6625232,4767634976-4767662856,2122995-2257010,1194-1754,779-1160,22-38,4961-6948,39-53,102-120,169741-245433,92902394-92956787,531-721,64-101,15596-20965,774184-943987,8395-11781,30178-47948,94338815-94398813";

fn main() {

    let ranges = INPUT.split(",").collect::<Vec<&str>>();
    let mut sum_ids: i64 = 0;

    for range in ranges {

        let (lower, upper) = range.split_once("-").unwrap();
        let lower_bound: i64 = lower.parse().unwrap();
        let upper_bound: i64 = upper.parse().unwrap();

        for pattern in lower_bound..=upper_bound {

            let current: String = pattern.to_string();
            let str_len = current.len();
            let factors = get_factors_naive(str_len);

            for factor in factors {

                let mut key: &str = "";
                let mut key_count = 0;
                let mut remaining: &str = &current;

                for i in 1..=str_len/factor {
                    let ( chunk, tail) = remaining.split_at(factor);

                    if i == 1 { key = chunk; }
                    else if chunk != key { break; }

                    remaining = tail;
                    key_count += 1;
                }

                if key_count == str_len/factor { 
                    sum_ids += pattern;
                    break;
                }
            }
        }
    }
    println!("{sum_ids}");

}

fn get_factors_naive(number:usize) -> Vec<usize> {

    let mut factors : Vec<usize> = Vec::new();

    for i in 1..=number {
        if i == number { continue; }
        if number % i == 0 { factors.push(i); }
    }

    return factors;

}