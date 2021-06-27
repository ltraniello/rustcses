
use std::io;


// WEIRD ALGORITHM - https://cses.fi/problemset/task/1068

pub fn solve_weird_algorithm<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n = scan.token();
    do_solve_weird_algorithm(n, out);
}

fn do_solve_weird_algorithm<W: io::Write>(mut n : u64, out: &mut W) {
    while n != 1 {
        write!(out, "{} ", n).ok();
        if n % 2 == 0 {
            n = n/2;
        } else {
            n = n*3 + 1;
        }
    }
    write!(out, "1").ok();
}

#[test]
fn test_weird() {
    let mut stdout = Vec::new();
    do_solve_weird_algorithm(3, &mut stdout);

    assert_eq!(stdout, b"3 10 5 16 8 4 2 1");
}

// MISSING NUMBER - https://cses.fi/problemset/task/1083

pub fn solve_missing_number<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n : i32 = scan.token();
    let numbers: Vec<i32> = scan.tokens();
    do_solve_missing_number(n, numbers, out);

}

fn do_solve_missing_number<W: io::Write>(n : i32, numbers : Vec<i32>, out: &mut W) {
    let mut found = vec![false; n as usize];
    for v in &numbers {
        found[(*v-1) as usize] = true
    }
    for (i, f) in found.iter().enumerate() {
        if !f {
            write!(out, "{}", i+1).ok();
        }
    }
}

#[test]
fn test_missing_number() {
    let mut stdout = Vec::new();
    let numbers = vec![2, 3, 1, 5];
    do_solve_missing_number(5, numbers, &mut stdout);
    assert_eq!(stdout, b"4");
}

// REPETITIONS - https://cses.fi/problemset/task/1069

pub fn solve_repetitions<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let dna: String = scan.token();
    do_solve_repetitions(dna, out);
}

fn do_solve_repetitions<W: io::Write>(dna : String, out: &mut W) {
    let trimmedDna = dna.trim();

    let mut longestSeq = 0;
    let mut currentChar = 'Z';
    let mut currentSeqLength = 0;
    for (_i, c) in trimmedDna.chars().enumerate() {
        if c != currentChar {
            currentChar = c;
            currentSeqLength = 1;
        } else {
            currentSeqLength+=1;
        }
        if currentSeqLength > longestSeq {
            longestSeq = currentSeqLength;
        }
    }
    write!(out, "{}", longestSeq).ok();
}

#[test]
fn test_repetitions() {
    let mut stdout = Vec::new();
    let dna = String::from("ATTCGGGA");
    do_solve_repetitions(dna, &mut stdout);
    assert_eq!(stdout, b"3");
}

// INCREASING ARRAY - https://cses.fi/problemset/task/1094

pub fn solve_increasing_array<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let _n: u64 = scan.token();

    let numbers : Vec<u64> = scan.tokens();

    let mut turnCount : u64 = 0;
    let mut m : u64 = 0;
    for v in numbers.iter() {
        m = std::cmp::max(m, *v);
        turnCount += m-*v;
    }
    write!(out, "{}", turnCount).ok();
}

// PERMUTATIONS - https://cses.fi/problemset/task/1070

pub fn solve_permutations<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n: u64 = scan.token();
    do_solve_permutations(n, out);
}

fn do_solve_permutations<W: io::Write>(n : u64, out: &mut W) {
    if n == 1 {
        write!(out, "1").ok();
        return
    }
    if n < 4 {
        write!(out, "NO SOLUTION").ok();
        return
    }
    if n == 4 {
        write!(out, "2 4 1 3").ok();
        return
    }

    let mut k = 1;
    let switch = (n+1)/2;
    for i in 1..n+1 {
        write!(out, "{} ", k).ok();
        k+=2;
        if i == switch {
            k = 2
        }
    }
}

#[test]
fn test_permutation() {
    let mut stdout = Vec::new();
    do_solve_permutations(3, &mut stdout);
    assert_eq!(stdout, b"NO SOLUTION");

    let mut stdout2 = Vec::new();
    do_solve_permutations(5, &mut stdout2);
    assert_eq!(stdout2, b"1 3 5 2 4 ");

}

// NUMBER SPIRAL - https://cses.fi/problemset/task/1071

pub fn solve_number_spiral<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let t: i64 = scan.token();
    let vec : Vec<crate::utils::Pair<u64>>= scan.pairs(t as u32);
    do_solve_number_spiral(vec, out);
}

fn do_solve_number_spiral<W: io::Write>(coords : Vec<crate::utils::Pair<u64>>, out: &mut W) {

    for coord in coords {
        let y = coord.x;
        let x = coord.y;
        let z = std::cmp::max(x,y);

        let res : u64;
        if y == z {
            if y%2 == 0 {
                res = y*y - x + 1;
            } else {
                res = (y-1)*(y-1)+1 + x -1;
            }
        } else {
            if x%2 == 1 {
                res = x*x - y + 1;
            } else {
                res = (x-1)*(x-1)+1 + y - 1;
            }
        }
        writeln!(out, "{}", res).ok();
    }
}

#[test]
fn test_number_spirals() {
    let mut stdout = Vec::new();
    let coords = vec!{crate::utils::Pair{x:2, y:3}, crate::utils::Pair{x:1, y:1}, crate::utils::Pair{x:4,y:2}};
    do_solve_number_spiral(coords, &mut stdout);
    assert_eq!(stdout, b"8\n1\n15\n");
}

// TWO KNIGHTS - https://cses.fi/problemset/task/1072

pub fn solve_two_knights<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n: i64 = scan.token();
    do_solve_two_knights(n, out);
}

fn do_solve_two_knights<W: io::Write>(n : i64, out: &mut W) {
    for k in 1..n+1 {
        let mut res : i64 = k*k*(k*k-1)/2;

        if k > 2 {
            res -= 4*(k-1)*(k-2);
        }
        writeln!(out, "{}", res).ok();
    }
}

#[test]
fn test_two_knights() {
    let mut stdout = Vec::new();
    do_solve_two_knights(8, &mut stdout);
    assert_eq!(stdout, b"0\n6\n28\n96\n252\n550\n1056\n1848\n");
}

// TWO SETS - https://cses.fi/problemset/task/1092

pub fn solve_two_sets<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n: i64 = scan.token();
    do_solve_two_knights(n, out);
}

fn do_solve_two_sets<W: io::Write>(n : i64, out: &mut W) {

    let sum = n*(n+1)/2;
    if sum%2 != 0 {
        writeln!(out, "NO").ok();
        return;
    }
    writeln!(out, "YES").ok();

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let mod4 = n%4;
    if mod4 == 3 {
        vec1.push(1);
        vec1.push(2);
        vec2.push(3);
        for k in 0..(n-1)/4 {
            vec1.push(4*k+4);
            vec2.push(4*k+5);
            vec2.push(4*k+6);
            vec1.push(4*k+7);
        }
    } else {
        vec2.push(1);
        vec1.push(2);
        vec1.push(3);
        vec2.push(4);
        for k in 0..(n-1)/4 {
            vec1.push(4*k+5);
            vec2.push(4*k+6);
            vec2.push(4*k+7);
            vec1.push(4*k+8);
        }
    }

    crate::utils::print_vec(&vec1, out);
    writeln!(out);
    crate::utils::print_vec(&vec2, out);
}

#[test]
fn test_two_sets() {
    let mut stdout = Vec::new();
    do_solve_two_sets(7, &mut stdout);
    assert_eq!(stdout, b"YES\n4\n1 2 4 7 \n3\n3 5 6 ");
}


// BITSTRING - https://cses.fi/problemset/task/1617

pub fn solve_bit_strings<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n: u64 = scan.token();
    do_solve_bit_strings(n, out);
}

fn do_solve_bit_strings<W: io::Write>(n : u64, out: &mut W) {
    let res_mod : u64 = 1000000007;
    let mut res : u64 = 1;
    for _k in 0..n {
        res = (2*res) % res_mod;
    }
    write!(out, "{}", res).ok();
}

#[test]
fn test_bitstrings() {
    let mut stdout = Vec::new();
    do_solve_bit_strings(3, &mut stdout);
    assert_eq!(stdout, b"8");
}

// TRAILING ZEROS - https://cses.fi/problemset/task/1618

pub fn solve_trailing_zeros<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n: u64 = scan.token();
    do_solve_trailing_zeros(n, out);
}

fn do_solve_trailing_zeros_slow<W: io::Write>(n : u64, out: &mut W) {
    let mut count_two : u64 = 0;
    let mut count_five : u64 = 0;
    for k in 1..n+1 {
        let mut a = k;
        while a%2 == 0 {
            count_two+=1;
            a=a/2;
        }
        while a%5 == 0 {
            count_five+=1;
            a=a/5;
        }
    }
    write!(out, "{}", std::cmp::min(count_two, count_five)).ok();
}

fn do_solve_trailing_zeros<W: io::Write>(n : u64, out: &mut W) {
    let mut res : u64 = 0;
    let mut power5 : u64 = 5;
    for _k in 1..n+1 {
        res += n/power5;
        power5 *= 5;
        if power5 > n {
            break;
        }
    }
    write!(out, "{}", res).ok();
}

#[test]
fn test_trailingzeros() {
    let mut stdout = Vec::new();
    do_solve_trailing_zeros(20, &mut stdout);
    assert_eq!(stdout, b"4");
    let mut stdout2 = Vec::new();
    do_solve_trailing_zeros_slow(20, &mut stdout2);
    assert_eq!(stdout2, b"4");
}

// COIN PILES - https://cses.fi/problemset/task/1754

pub fn solve_coin_piles<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let t = scan.token();
    let vec = scan.pairs(t);
    do_solve_coin_piles(vec, out);
}

fn do_solve_coin_piles<W: io::Write>(pairs : Vec<crate::utils::Pair<u64>>, out: &mut W) {

    for p in pairs {
        let a = std::cmp::min(p.x, p.y);
        let b = std::cmp::max(p.x, p.y);

        if (2*b-a)%3 != 0 || b-2*(2*b-a)/3 < 0 {
            writeln!(out, "NO").ok();
        } else {
            writeln!(out, "YES").ok();
        }
    }
}

#[test]
fn test_coinpiles() {
    let mut stdout = Vec::new();
    let pairs = vec!{crate::utils::Pair{x:2, y:1}, crate::utils::Pair{x:2, y:2}, crate::utils::Pair{x:3,y:3}};
    do_solve_coin_piles(pairs, &mut stdout);
    assert_eq!(stdout, b"YES\nNO\nYES\n");
}

// GRID PATH - https://cses.fi/problemset/task/1623

const SIZE: usize = 7;
const SQQIZE: usize = SIZE*SIZE;
const FIRST_POS: usize = 0;
const LAST_POS: usize = SIZE*(SIZE-1);
const PATHLEN: usize = SIZE*SIZE-1;

#[inline]
fn is_available(visited : &[[bool; SIZE]], i : usize, j : usize) -> bool {
    i < SIZE && j < SIZE && !visited[i][j]
}

struct Counter {
    count: u64,
    rec: u64,
}

fn rec_solve_grid_path(count : &mut Counter, path : &[u8], visited : &mut[[bool; SIZE]], gi : usize, gj : usize, i : usize, lastchar : u8) {
    if i == 48 {
        if gi == 0 && gj == SIZE-1 {
            count.count += 1;
        }
        return;
    }
 
    if gi == 0 && gj == SIZE-1 {
        return;
    }
    visited[gi][gj] = true;
 
    if lastchar != b'U' && is_available(visited, gi, gj+1) && (path[i] == b'D' || path[i] == b'?' && !(!is_available(visited, gi, gj+2) && is_available(visited, gi+1, gj+1) && is_available(visited, gi-1, gj+1))) {
        rec_solve_grid_path(count, path, visited, gi, gj+1, i+1, b'D');
    }
    if lastchar != b'R' && is_available(visited, gi-1, gj) && (path[i] == b'L' || path[i] == b'?' && !(!is_available(visited, gi-2, gj) && is_available(visited, gi-1, gj-1) && is_available(visited, gi-1, gj+1))) {
        rec_solve_grid_path(count, path, visited, gi-1, gj, i+1, b'L');
    }
    if lastchar != b'L' && is_available(visited, gi+1, gj) && (path[i] == b'R' || path[i] == b'?' && !(!is_available(visited, gi+2, gj) && is_available(visited, gi+1, gj-1) && is_available(visited, gi+1, gj+1))) {
        rec_solve_grid_path(count, path, visited, gi+1, gj, i+1, b'R');
    }
    if lastchar != b'D' && is_available(visited, gi, gj-1) && (path[i] == b'U' || path[i] == b'?' && !(!is_available(visited, gi, gj-2) && is_available(visited, gi+1, gj-1) && is_available(visited, gi-1, gj-1))) {
        rec_solve_grid_path(count, path, visited, gi, gj-1, i+1, b'U');
    }
    visited[gi][gj] = false;
}

fn do_solve_grid_path(path : &[u8]) -> u64 {

    let mut visited = [[false; SIZE]; SIZE];

    let mut c = Counter {
        count: 0,
        rec: 0,
    };

    rec_solve_grid_path(&mut c, &path, &mut visited, FIRST_POS, 0, 0, b'D');
    c.count
}

fn solve_grid_path<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let path: String = scan.token();
    let path: &[u8] = path.as_bytes();
    let ans = do_solve_grid_path(&path);
    writeln!(out, "{}", ans).ok();
}

#[test]
fn test_gridpath() {
    assert_eq!(do_solve_grid_path(b"??????R??????U??????????????????????????LD????D?"), 201);
}

// PALINDROME - https://cses.fi/problemset/task/1755

pub fn solve_palindrome_reorder<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let raws: String = scan.token();
    do_solve_palindrome_reorder(raws, out);
}

fn do_solve_palindrome_reorder<W: io::Write>(raws : String, out: &mut W) {
    let s = raws.trim();
    let mut charcount = vec![0; 26];
    for c in s.chars() {
        charcount[crate::utils::char_to_u8(c) as usize] += 1;
    }

    let mut oddchar = 'a';
    let mut halfout = vec![0; s.len()/2 as usize];
    let mut halfoutidx : usize = 0;
    for (c, count) in charcount.iter().enumerate() {
        if count%2 != 0 {
            if oddchar != 'a' || s.len()%2 == 0 {
                print!("NO SOLUTION");
                return;
            }
            oddchar = crate::utils::u8_to_char(c as u8);
        } else {
            for _i in 0..count/2 {
                halfout[halfoutidx] = c;
                halfoutidx+=1;
            }
        }
    }

    for c in halfout.iter() {
        write!(out, "{}", crate::utils::u8_to_char(*c as u8));
    }
    if oddchar != 'a' {
        write!(out, "{}", oddchar);
    }
    halfout.reverse();
    for c in halfout.iter() {
        write!(out, "{}", crate::utils::u8_to_char(*c as u8));
    }
}

#[test]
fn test_palindrome_reorder() {
    let mut stdout = Vec::new();
    do_solve_palindrome_reorder("AAAACACBA".to_string(), &mut stdout);
    assert_eq!(stdout, b"AAACBCAAA");
}

// APPLE DIVISION - https://cses.fi/problemset/task/1755

pub fn solve_apple_division<R: io::BufRead, W: io::Write>(scan: &mut crate::utils::Scanner<R>, out: &mut W) {
    let n = scan.token();
    let weights = scan.tokens();
    do_solve_apple_division(n, weights, out);
}

fn do_solve_apple_division<W: io::Write>(n : i32, weights : Vec<i32>, out: &mut W) {
    let mut sum = 0;
    for w in weights.iter() {
        sum += w;
    }

    let mut maxgroupsum = 0;
    for subset in 0..1<<n {
        let mut localsum = 0;
        for k in 0..n {
            if subset>>k&1 != 0 {
                localsum += weights[k as usize]
            }
        }
        if localsum <= sum/2 {
            maxgroupsum = std::cmp::max(localsum, maxgroupsum);
        }
    }

    writeln!(out, "{}", sum-2*maxgroupsum);
}

#[test]
fn test_apple_division() {
    let mut stdout = Vec::new();
    do_solve_apple_division(5, vec!{3, 2, 7, 4, 1}, &mut stdout);
    assert_eq!(stdout, b"1\n");
}
