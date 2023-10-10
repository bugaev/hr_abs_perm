/*
const N: i32 = 770;
const K: i32 = 380;
*/

/*
const N: i32 = 2;
const K: i32 = 0;
*/


const N: i32 = 30;
const K: i32 = 5;


// use is_odd::IsOdd;
use std::collections::HashMap;
use std::process::exit;


fn main() {

    println!("Ranges:");
    // This uses an analytical approach.  It works with any large N and K.
    let res = absolutePermutation(N, K);
    println!("Solution:");
    for el in &res {
        print!("{} ", el);
    }
    println!("\n");
    
    exit(0);

    // The code below is fully functional.  It is using recursions. For that reason, it will be slow for N >~ 20.
    let mut taken: HashMap<i32, bool> = HashMap::new(); // Do I need to initialize this?
    let mut res: Vec<i32> = Vec::new(); // Do I need to initialize this?
    let mut all_res: Vec<Vec<i32>> = Vec::new();
    println!("Hello, world!");
    perm(&mut all_res, &mut res, &mut taken, 1, N, K);
    // all_res = vec![vec![2, 2], vec![1, 2], vec![8, 2], vec![10, 2]];
    all_res.sort();
    all_res.dedup();
    println!("all_res: {:?}", all_res);
}

fn absolutePermutation(n: i32, k: i32) -> Vec<i32> {
    /*
For the absolute permutation to exits, its members have to meet the condition below.
The bottom row shows index sub ranges in 1...N where value = index + k ("+") and where
value = index - k ("-").  If the first members cannot fall into a "+" subrange, then the absolute permutation is not possible.
       "+"                     "-"                        "+"                 "-"
N - 4*k + 1 ... N - 3*k, N - 3*k + 1 ... N - 2*k, N - 2*k + 1 ... N - k, N - k + 1 ... N

This is equivalent to N - c*k + 1 = 1, c = N / k, where c is even. If c is not even, the permutation doesn't exist.
*/

if k == 0 {
    return (1..=N).collect();
}
let c0: i32 = n / k;
let k_divides_n = 0 == n % k;
if !k_divides_n {
    return vec![-1];
}
let c0_is_odd = 1 == c0 % 2;
// println!("is_odd {}", c0_is_odd);
let mut c = c0;
let mut plus: bool = true;
let mut res: Vec<i32> = Vec::new();
if c0_is_odd {
    //println!("-1");
} else {
    while c > 0 {
        let a = N - c * K + 1;
        let b = N - (c - 1) * K;

        let op = if plus { "+" } else { "-" };
        
        println!("{}{} ... {}", op, a, b);
        for i in a..=b {
            let val = i + if plus { K } else { -K };
            res.push(val);
        }
        plus = !plus;
        c = c - 1;
    }
}
if res.len() > 0 {
    return res;
} else {
    return vec![-1];
}
}

fn perm(
    all_res: &mut Vec<Vec<i32>>,
    res: &mut Vec<i32>,
    taken: &mut HashMap<i32, bool>,
    i: i32,
    n: i32,
    k: i32,
) {
    let mut opt: Vec<i32> = Vec::new();
    let i_m_k = i - k;
    if i_m_k > 0 {
        if !is_taken(taken, i_m_k) {
            // taken.insert(i_m_k, true); // These 2 lines should be moved to the while loop.
            opt.push(i_m_k);
        }
    }
    let i_p_k = i + k;
    if i_p_k <= n {
        if !is_taken(taken, i_p_k) {
            // taken.insert(i_p_k, true); // These 2 lines should be moved to the while loop.
            opt.push(i_p_k);
        }
    }
    if opt.len() == 0 {
        // println!("ANSWER: -1");
        println!("Backtracking at i: {}", i);
        return;
    }

    let mut attempts = 0;
    while let Some(val) = opt.pop() {
        attempts += 1;
        if all_res.len() == 1 {
            break;
        }
        if attempts > 1 {
            println!("Trying again at i = {} with val = {}", i, val);
        }
        res.push(val);
        taken.insert(val, true);

        if i == n {
            // println!("ANSWER: {:?}", res);
            all_res.push(res.to_vec());
        } else {
            perm(all_res, res, taken, i + 1, n, k);
        }
        taken.remove(&val);
        res.pop();
    }
}

fn is_taken(taken: &mut HashMap<i32, bool>, key: i32) -> bool {
    if let Some(_) = taken.get(&key) {
        return true;
    }
    return false;
}
