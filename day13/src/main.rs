use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
    let input_str = std::fs::read_to_string("input.txt")?;

    let (timestamp, buses) = parse(&input_str)?;

    part1(timestamp, &buses);
    part2(&buses);

    Ok(())
}

fn parse(s: &str) -> Result<(usize, Vec<(usize, usize)>)> {
    let mut lines = s.lines();
    let timestamp_str = lines.next().ok_or_else(|| anyhow!("missing timestamp line"))?;
    let timestamp = timestamp_str.parse().with_context(|| format!("could not parse timestamp '{}'", timestamp_str))?;
    let buses_str = lines.next().ok_or_else(|| anyhow!("missing bus IDs"))?;
    let buses = buses_str.split(',')
        .enumerate()
        .filter(|&(_, s)| s != "x")
        .map(|(i, s)| s.parse().map(|v| (i, v)).with_context(|| format!("could not parse bus ids '{}'", buses_str)))
        .collect::<Result<_, _>>()?;
    
    Ok((timestamp, buses))

}

fn part1(timestamp: usize, buses: &Vec<(usize, usize)>) {
    let (id, wait_time) = buses.iter()
        .map(|(_, id)| (id, (id - timestamp % id) % id))
        .min_by_key(|&(_, t)| t)
        .unwrap();
    
    let answer = id * wait_time;
    println!("part1 = {}", answer);
}


fn part2(buses: &Vec<(usize, usize)>) {
    for (i, bus) in buses {
        println!("{} {}", i , bus);
    }

    let ps: Vec<i64> = buses.iter().map(|&(_, b)| b as i64).collect();
    let xs: Vec<i64> = buses.iter().map(|&(i, b)| ((b as i64) - (i as i64)) % (b as i64) ).collect();

    let answer = chinese_remainder_theorem(&ps, &xs);
    println!("part2 = {}", answer);
}

// https://math.stackexchange.com/a/2060259
fn chinese_remainder_theorem(ps: &Vec<i64>, xs: &Vec<i64>) -> i64 {
    let big_p: i64 = ps.iter().product();
    let mut big_x = 0;
    for (&p, &x) in ps.iter().zip(xs.iter()) {
        let (_, b, r) = extended_gcd(p, big_p/p);
        assert_eq!(r, 1, "expected r to be 1!");
        let u = big_p/p * b;
        big_x += x * u;
    }

    let mut answer = big_x;
    while answer < 0 {
        answer += big_p;
    }

    answer %= big_p;
    answer
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    while r != 0 {
        let quotient = old_r / r;
        let r_vals = (r, old_r - quotient * r);
        let s_vals = (s, old_s - quotient * s);
        let t_vals = (t, old_t - quotient * t);

        old_r = r_vals.0;
        r = r_vals.1;
        old_s = s_vals.0;
        s = s_vals.1;
        old_t = t_vals.0;
        t = t_vals.1;
    }

    (old_s, old_t, old_r)
}


