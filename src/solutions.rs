use std::io;
use std::io::*;


fn main() {
    solve_weird_algorithm(n);
}

fn solve_pe() {
    let n = read_input().trim().parse::<u64>().expect("wrong input");

    for i in 0..1<<n {
        for k in 0..n {
            if i>>k&1 != 0 {
                print!("{} ", k);
            }
        }
        println!("");
    }
}

fn solve_chess() {
    let mut reserved = vec![false; 64];

    for i in 0..8 {
        let raws = read_input();
        let s = raws.trim();
        for (j, c) in s.chars().enumerate() {
            if c == '*' {
                reserved[8*i+j] = true;
            }
        }
    }

    let mut busydiag = vec![false; 15];

    let mut count = 0;
    let mut perm = [0, 1, 2, 3, 4, 5, 6, 7];
    loop {
        let mut valid : bool = true;
        for (i, p) in perm.iter().enumerate() {
            if reserved[8*i+*p] {
                valid = false;
                break;
            }
            let diag_id = (15+std::cmp::max(i, *p) - std::cmp::min(i, *p))%15;
            if busydiag[diag_id] == true {
                valid = false;
                break;
            }
            busydiag[diag_id] = true;
        }

        if valid {
            count += 1;
        }

        for d in busydiag.iter_mut() {
            *d = false;
        }
        if !perm.next_permutation() {
            break;
        }
    }
    println!("{}", count);
}