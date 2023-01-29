use std::{
    collections::{BTreeSet, HashMap, HashSet},
    vec,
};

use petgraph::{prelude::UnGraphMap, dot::{Config, Dot}};

pub fn solution() {
    println!("Solution for 1627");
    let n = 6;
    let threshold = 1;
    let queries = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    let are_connected = are_connected(n, threshold, queries);
    println!("Output: {:?}", are_connected);
}

pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let mut output: Vec<bool> = Vec::with_capacity((n / 2) as usize);
    let graph = generate_graph(n, threshold);

    println!("{:#?}", graph);
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // for query in queries.iter() {
    //     output.push(false);
    //     let first = query[0];
    //     let last = query[1];

    //     let first_divisors = match cached_results.get(&first) {
    //         Some(divisors) => divisors.clone(),
    //         None => {
    //             let divisors = get_divisors(first);
    //             cached_results.insert(first, divisors.clone());
    //             divisors
    //        },
    //     };

    //     // println!("first_divisors: {:?}", first_divisors);

    //     let last_divisors = match cached_results.get(&last) {
    //         Some(divisors) => divisors.clone(),
    //         None => {
    //             let divisors = get_divisors(last);
    //             cached_results.insert(last, divisors.clone());
    //             divisors
    //         }
    //     };

    //     // println!("second_divisors: {:?}", last_divisors);

    //     if first_divisors.len() > last_divisors.len() {
    //         for divisor in last_divisors.iter() {
    //             if divisor <= &threshold {
    //                 continue;
    //             }
    //             match first_divisors.get(divisor) {
    //                 Some(_) => {
    //                     output[i] = true;
    //                     break;
    //                 },
    //                 None => (),
    //             }
    //         }
    //     } else {
    //         for divisor in first_divisors.iter() {
    //             if divisor <= &threshold {
    //                 continue;
    //             }
    //             match last_divisors.get(divisor) {
    //                 Some(_) => {
    //                     output[i] = true;
    //                     break;
    //                 },
    //                 None => (),
    //             }
    //         }
    //     }
    //     i += 1;
    // }
    output
}

fn generate_graph(n: i32, threshold: i32) -> UnGraphMap<i32, ()> {
    let mut graph: UnGraphMap<i32, ()> = UnGraphMap::new();

    for i in 1..n + 1 {
        let divisors = get_divisors(i).split_off(&(threshold + 1));
        for divisor in divisors {
            graph.add_edge(i, divisor, ());
        }
    }

    graph
}

fn get_divisors(number: i32) -> BTreeSet<i32> {
    // 50
    // [1,2,5,10,25,50]
    let mut divisors: BTreeSet<i32> = BTreeSet::from([1, number]);
    let mut i = 2;
    let mut last_divisor = number;
    loop {
        if i >= last_divisor {
            break;
        }
        if number % i == 0 {
            last_divisor = number / i;
            divisors.insert(i);
            divisors.insert(last_divisor);
        }
        i += 1;
    }
    // println!("number: {}\tdivisors: {:?}", number, divisors);
    divisors
}
