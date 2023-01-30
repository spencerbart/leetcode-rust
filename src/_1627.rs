use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    vec,
};

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

    for query in queries {
        let a = query[0];
        let b = query[1];
        output.push(bfs(&graph, a, b))
    }
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

fn bfs(graph: &HashMap<i32, BTreeSet<i32>>, a: i32, b: i32) -> bool {
    let graph = graph.clone();
    let found = false;
    // println!("graph: {:?}", graph);
    // println!("a: {}\tb: {}", a, b);

    let mut visited: HashSet<i32> = HashSet::from([a]);
    let mut queue: VecDeque<i32> = VecDeque::new();

    queue.push_back(a);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        let connections = graph.get(&node).unwrap();
        // println!("node: {:?}\tconnections: {:?}\t", node, connections);
        
        for connection in connections {
                if connection == &b {
                    // println!("true called");
                    return true;
                }

                if visited.get(connection).is_none() {
                    visited.insert(connection.clone());
                    queue.push_back(connection.clone());
                }
        }
    }

    found
}

fn generate_graph(n: i32, threshold: i32) -> HashMap<i32, BTreeSet<i32>> {
    let mut graph: HashMap<i32, BTreeSet<i32>> = HashMap::new();

    for i in 1..n + 1 {
        let divisors = get_divisors(i).split_off(&(threshold + 1));
        graph.insert(i, divisors.clone());
        for divisor in divisors {
            let connections  = graph.get_mut(&divisor).unwrap();
            connections.insert(i);
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::get_divisors;

    #[test]
    // #[ignore]
    fn test_get_divisors() {
        // let divisors = get_divisors(50);
        // let correct_divisors = BTreeSet::from([1,2,5,10,25,50]);
        // let divisors = get_divisors(4);
        // let correct_divisors = BTreeSet::from([1,2,4]);
        let divisors = get_divisors(9);
        let correct_divisors = BTreeSet::from([1, 3, 9]);
        assert_eq!(correct_divisors, divisors);
    }
}
