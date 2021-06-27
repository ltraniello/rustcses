use std::io;
use std::collections::hash_map::{HashMap, Entry};

// DISTINCT NUMBERS - https://cses.fi/problemset/task/1621

pub fn solve_distinctnumbers<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let _n : u64 = scan.token();
    let mut numbers = scan.tokens();
    let c = do_solve_distinctnumbers(&mut numbers);
    write!(out, "{}", c);
}

fn do_solve_distinctnumbers(numbers : &mut Vec<u64>) -> u64 {

    numbers.sort();
    let mut count : u64 = 0;
    let mut prev : u64 = 0;
    for v in numbers.iter() {
        if *v != prev {
            prev = *v;
            count += 1;
        }
    }
    count
}

#[test]
fn test_distinctnumbers() {
    let mut n = vec!{2,3,2,2,3};
    assert_eq!(do_solve_distinctnumbers(&mut n), 2);
}


// APARTMENTS - https://cses.fi/problemset/task/1084

fn solve_appartments<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let _n: i64 = scan.token();
    let _m: i64 = scan.token();
    let k: i64 = scan.token();

    let mut expected: Vec<i64> = scan.tokens();
    let mut size: Vec<i64> = scan.tokens();

    let ans = do_solve_apartments(k, &mut expected, &mut size);
    writeln!(out, "{}", ans).ok();
}

fn do_solve_apartments(k : i64, expected : &mut Vec<i64>, size : &mut Vec<i64>) -> u64 {

    expected.sort();
    size.sort();
    let mut count : u64 = 0;

    let mut exi : usize = 0;
    let mut si : usize = 0;
    while exi < expected.len() && si < size.len() {
        //println!("{} {}", size[si], expected[exi]);
        if size[si] < expected[exi]-k {
            si+=1;
            if si >= size.len() {
                break;
            }
            continue;
        }
        if size[si] > expected[exi]+k {
            exi+=1;
            if exi >= expected.len() {
                break;
            }
            continue;
        }
        count += 1;
        exi+=1;
        si+=1;
    }
    count
}

#[test]
fn test_solve_appartments() {
    let mut size = vec!{30, 60, 75};
    let mut ex = vec!{60, 45, 80, 60};
    assert_eq!(do_solve_apartments(5, &mut ex, &mut size), 2);
}

// FERRIS WHEEL - https://cses.fi/problemset/task/1090

fn solve_ferris<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let _n: i64 = scan.token();
    let x: i64 = scan.token();

    let mut size: Vec<i64> = scan.tokens();

    let ans = do_solve_ferris(x, &mut size);
    write!(out, "{}", ans);
}

fn do_solve_ferris(x : i64, size : &mut Vec<i64>) -> u64 {

    let mut compl: Vec<i64> = size.iter().map(|s| x-s).collect();

    compl.sort();
    size.sort();

    let mut count : u64 = 0;

    let mut ci : usize = 0;
    let mut si : usize = 0;
    while ci < compl.len() && si < size.len() {
        if size[si] <= compl[ci] {
            si+=1;
            continue;
        }
        count += 1;
        ci+=1;
        si+=1;
    }
    count
}

#[test]
fn test_ferris() {
    let mut s = vec!{7, 2, 3, 9};
    assert_eq!(do_solve_ferris(10, &mut s), 3);
}

// CONCERT TICKET - https://cses.fi/problemset/task/1091

fn solve_concert_ticket<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let _n: i64 = scan.token();
    let _m: i64 = scan.token();

    let mut prices: Vec<i64> = scan.tokens();
    let mut customers: Vec<i64> = scan.tokens();

    do_solve_concert_ticket(&mut prices, &mut customers, out);
}

fn do_solve_concert_ticket<W: io::Write>(prices : &mut Vec<i64>, customers : &mut Vec<i64>, out: &mut W) {
    prices.sort_by(|a, b| b.cmp(a));

    for c in customers {
        let mut max_price = -1;
        let mut used_index = 0;
        for (i, p) in prices.iter().enumerate() {
            if *p >= 0 && *p <= *c {
                max_price = *p;
                used_index = i;
                break;
            }
        }
        if max_price != -1 {
            prices[used_index] = -1;
        }
        writeln!(out, "{}", max_price).ok();
    }
}

#[test]
fn test_concert_ticket() {
    let mut stdout = Vec::new();
    let mut p = vec!{5, 3, 7, 8, 5};
    let mut c = vec!{4, 8, 3};
    do_solve_concert_ticket(&mut p, &mut c, &mut stdout);
    assert_eq!(stdout, b"3\n8\n-1\n");
}

// RESTAURANT CUSTOMERS - https://cses.fi/problemset/task/1619

fn solve_restaurant_customers<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n: i64 = scan.token();

    let times = scan.pairs(n as u32);

    let mut arrival_t = Vec::new();
    let mut leaving_t = Vec::new();

    for t in times {
        arrival_t.push(t.x);
        leaving_t.push(t.y);
    }

    let c = do_solve_restaurant_customers(&mut arrival_t, &mut leaving_t);
    write!(out, "{}", c).ok();
}

fn do_solve_restaurant_customers(arrival_t : &mut Vec<u64>, leaving_t : &mut Vec<u64>) -> u64 {
    arrival_t.sort();
    leaving_t.sort();

    let mut c = 0;
    let mut maxc = 0;
    let mut l_i = 0;

    for t in arrival_t {
        c += 1;
        while leaving_t[l_i] < *t {
            c -= 1;
            l_i+=1;
        }
        maxc = std::cmp::max(maxc, c);
    }
    maxc
}

#[test]
fn test_restaurant() {
    let mut a = vec!{5, 2, 3};
    let mut l = vec!{8, 4, 9};
    assert_eq!(do_solve_restaurant_customers(&mut a, &mut l), 2);
}

// MOVIE FESTIVAL - https://cses.fi/problemset/task/1629

fn solve_movie_festival<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n: i64 = scan.token();
    let mut movies : Vec<crate::utils::Pair<u64>> = scan.pairs(n as u32);
    movies.sort_by(|a, b| a.x.cmp(&b.x));

    let c = do_solve_movie_festival(&movies, -1, 0);
    write!(out, "{}", c).ok();
}

fn do_solve_movie_festival(movies : &Vec<crate::utils::Pair<u64>>, mut last_film_index : i32, movie_seen : u64) -> u64 {

    let mut next_available_movie_index = 0;
    if last_film_index >= 0 {
        let min_start = movies[last_film_index as usize].y;
        next_available_movie_index = last_film_index;
        while next_available_movie_index < movies.len() as i32 && movies[next_available_movie_index as usize].x < min_start {
            next_available_movie_index +=1;
        }
    } else {
        next_available_movie_index = 0;
    }
    return std::cmp::max(
        do_solve_movie_festival(movies, next_available_movie_index, movie_seen+1),
        do_solve_movie_festival(movies, next_available_movie_index+1, movie_seen)
    );
}

#[test]
fn test_movie_festival() {
    let mut a = vec!{5, 2, 3};
    let mut l = vec!{8, 4, 9};
    assert_eq!(do_solve_restaurant_customers(&mut a, &mut l), 2);
}

// SUM OF TWO VALUES - https://cses.fi/problemset/task/1640

fn solve_twovalues<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let _n: i64 = scan.token();
    let x: i64 = scan.token();
    let values = scan.tokens();

    let (u, v) = do_solve_twovalues(x, values);
    if u == 0 && v == 0 {
        write!(out, "IMPOSSIBLE").ok();
    } else {
        write!(out, "{} {}", u, v).ok();
    }
}

fn do_solve_twovalues(val : i64, values : Vec<i64>) -> (usize, usize) {
    let comp_values : Vec<i64> = values.iter().map(|x| val - x).rev().collect();
    let mut comp_lookup : HashMap<i64, usize> = HashMap::with_capacity(values.len());
    for (i, v) in comp_values.iter().enumerate() {
        comp_lookup.insert(*v, i);
    }
    for (i, v) in values.iter().enumerate() {
        match comp_lookup.get(v) {
            Some(&j) => return (i+1, j+1),
            _ => continue,
        }
    }
    (0,0)
}

#[test]
fn test_twovalues() {
    let (u,v) = do_solve_twovalues(8, vec!{2, 7, 5, 1});
    assert_eq!(u,4);
    assert_eq!(u,2);
}
