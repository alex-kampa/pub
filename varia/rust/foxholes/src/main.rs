mod wpath;

use std::collections::HashMap;
use wpath::WPath;


fn main() {

    // start time measurement
    let before = std::time::Instant::now();

    // cycle through all possible strategies, applying all possible paths
    //
    println!("\n--- brute force ---");
    brute_force(5, 6);

    // show some paths
    println!("\n--- show paths (5 holes, 3 days) ---");
    let paths = get_paths(5, 3);
    print_paths(paths);

    // check a strategy
    println!("\n--- check a strategy (3 holes, 2 days) ---");
    let paths = get_paths(3, 2);
    let s: Vec<u8> = vec![2, 2];
    check_strategy(&paths, &s, None);

    // check another strategy
    println!("\n--- check another strategy (5 holes, 6 days) ---");
    let paths = get_paths(5, 6);
    let s: Vec<u8> = vec![2, 3, 4, 2, 3, 4];
    check_strategy(&paths, &s, None);

    // check more strategies
    println!("\n--- check more strategies (5 holes, 7 days) ---");
    let paths = get_paths(5, 7);
    let s: Vec<u8> = vec![2, 3, 4, 2, 3, 4];
    check_strategy(&paths, &s, None);
    let s: Vec<u8> = vec![4, 2, 3, 4, 2, 3, 4];
    check_strategy(&paths, &s, None);
    let s: Vec<u8> = vec![2, 2, 3, 4, 4, 3, 2];
    check_strategy(&paths, &s, None);
    let s: Vec<u8> = vec![4, 2, 3, 4, 4, 3, 2];
    check_strategy(&paths, &s, None);
    let s: Vec<u8> = vec![2, 2, 3, 4, 2, 3, 4];
    check_strategy(&paths, &s, None);

    // check a strategy
    println!("\n--- check naive strategy (5 holes, 6 days) ---");
    let paths = get_paths(5, 6);
    let s: Vec<u8> = vec![3, 2, 3, 2, 3, 2];
    check_strategy(&paths, &s, Some(true));

    // check a strategy
    println!("\n--- check naive strategy (5 holes, 10 days) ---");
    let paths = get_paths(5, 10);
    let s: Vec<u8> = vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2];
    check_strategy(&paths, &s, None);

    // print probabilities depending on day 
    println!("\n--- some probabilities ---");
    for i in 1..6 {
        println!("Distribution of 5 holes on day {}", i);
        let paths = get_paths(5, i);
        let distribution = get_distribution(&paths);
        print_distribution(distribution);
    }

    // show how much time has elapsed
    println!("\nElapsed time: {:.2?}", before.elapsed());
}

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

// Get distribution of fox positions 

fn get_distribution(wpaths: &Vec<WPath>) -> HashMap<u8, f64> {
    let mut res: HashMap<u8, f64> = HashMap::new();
    for wpath in wpaths {
        let last: u8 = *wpath.path.last().unwrap();
        let prob: f64 = wpath.prob;
        *res.entry(last).or_insert(0.0) += prob;
    }
    res
}

fn print_distribution(distribution: HashMap<u8, f64>) {
    let mut keys: Vec<u8> = distribution.keys().cloned().collect();
    keys.sort();
    for key in keys {
        println!("Hole: {}, Probability: {:.4}", key, distribution.get(&key).unwrap());
    }
}

// Generates all possible paths for a given number of holes and a given length


fn get_paths(holes: u8, path_length: u8) -> Vec<WPath> {
    
    // initialisation: paths of length one 
    //
    let prob = 1.0 / holes as f64;
    let mut paths: Vec<WPath> = vec![];
    for i in 0..holes {
        paths.push(WPath::new(vec![i+1], prob));
    }

    // compute all paths of length LEN
    //
    for _ in 1..path_length {
        paths = extend_paths(paths, &holes);
    }
    println!("Generated {:?} paths of length {:?} for {:?} holes", paths.len(), path_length, holes);
    
    paths
}

fn print_paths(wpaths: Vec<WPath>) {
    for wpath in wpaths {
        println!("{}", wpath);
    }
}

// Takes a vector of paths and extends them one step

fn extend_paths(configs: Vec<WPath>, holes: &u8) -> Vec<WPath> {
    
    let mut extended: Vec<WPath> = Vec::new();

    for wpath in configs {
        let last = *wpath.path.last().unwrap();
        let prob: f64 = wpath.prob;
        if last == 1 {
            let mut v = wpath.path.to_vec();
            v.push(2);
            extended.push(WPath::new(v, prob));
        } else if last == *holes {
            let mut v = wpath.path.to_vec();
            v.push(*holes - 1);
            extended.push(WPath::new(v, prob));
        } else {
            let mut v1 = wpath.path.to_vec();
            let mut v2 = wpath.path.to_vec();
            v1.push(last - 1);
            v2.push(last + 1);
            extended.push(WPath::new(v1, prob/2.0));
            extended.push(WPath::new(v2, prob/2.0));
        }
    }
    
    extended
    
}

fn check_strategy(paths: &Vec<WPath>, strat: &Vec<u8>, verbose: Option<bool>) {

    let verbose = verbose.unwrap_or(false);

    let _path_count: u32 = paths.len() as u32;
    let path_length: usize= paths[0].path.len();

    let mut winner: bool = true;
    let mut wsteps: f64 = 0.0;
    let mut max: u32 = 0;
    let mut fails: u32 = 0;

    println!("  >> Strategy {:?}", strat);

    for path in paths {
        let prob = path.prob;
        let res = check_common(strat, &path.path);
        if res == 9999 {
            winner = false;
            fails += 1;
            wsteps = wsteps + prob * ((path_length + 1) as f64);
            if verbose {
                println!("Does not catch: {:?}", &path.path);
            }
        } else {
            wsteps = wsteps + prob * ((res + 1) as f64);
            max = std::cmp::max(max, res + 1);
        }
    }

    println!("  >> Strategy summary for {:?}", strat);
    if winner {
        println!("  >> This is a winning strategy ({} fails)", fails);
        println!("  >> Average number of steps taken is {:.4} (max {})", wsteps, max);
    } else {
        println!("  >> This is not a winning strategy ({} fails)", fails);
        println!("  >> Average number of steps taken is >= {:.4} (max {})", wsteps, max);
    }
    
}

fn brute_force(holes: u8, len: u8) {
    
    let paths = &get_paths(holes, len);
    
    let path_count: u32 = paths.len() as u32;
    let strategy_count: u32 = u32::pow(holes as u32, len as u32); 
    
    // cycle through all possible strategies, applying all possible paths
    //
    let mut strat: Vec<u8> = vec![1; usize::from(len)];
    let mut winner: bool;
    let mut wsteps: f64;
    let mut max: u32;
    let mut count: u32 = 0;

    loop {
        //
        let mut stats: HashMap<u32, u32> = HashMap::new();
        let mut stats2: HashMap<u32, f64> = HashMap::new();
        // println!("=== Stragetgy: {:?}", strat);
        winner = true;
        wsteps = 0.0;
        max = 0;
        count = count + 1;
        for p in paths {
            let res = check_common(&strat, &p.path);
            let prob = p.prob;
            if res == 9999 {
                winner = false;
            } else {
                wsteps = wsteps + prob * ((res + 1) as f64);
                max = std::cmp::max(max, res + 1);
                *stats.entry(res + 1).or_insert(0) += 1;
                *stats2.entry(res + 1).or_insert(0.0) += prob;
            }
            // println!("  path {:?} results in {:?}", p, res);
        }
        if winner == true {
            println!("  << Strategy {:?} is a winner (max {:?})>>", strat, max);
            println!("     Average number of steps taken {:.3}", wsteps);
            print!("     Stats ");
            for i in 1..=len {
                let pc = stats2.get(&(i as u32)).unwrap_or(&0.0) * 100.0;
                print!("{}=({})({:.3}%) ", i, stats.get(&(i as u32)).unwrap_or(&0), pc);
            }
            println!("");
        }
        
        // increment the srategy
        strat = increment_strat(strat, holes);
        
        // break if we're back to [1, ..., 1]
        if  strat.iter().all(|&x| x == 1) {
            break
        }  
    }
    println!("We have tested {:} strategies (expected {:?})", tfs(count), tfs(strategy_count));
    println!("Total iterations {:} x {:} = {:}", tfs(strategy_count), tfs(path_count), tfs(strategy_count * path_count));

}

// format integers with commas
// Cf (16) in https://stackoverflow.com/questions/26998485/is-it-possible-to-print-a-number-formatted-with-thousand-separator-in-rust

fn tfs(x: u32) -> String {
    
    let res = x.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",");
    
    res
    
}

// returns the smallest index for which the 2 arrays have a common element
// returns 9999 if there is no such element

fn check_common(v1: &Vec<u8>, v2: &Vec<u8>) -> u32 {
    //if v1.len() != v2.len() {
    //    println!("Length warning in check_common: {:?} - {:?}", v1, v2)
    //}
    for i in 0..v1.len() {
        if v1[i] == v2[i] {
            return i as u32;
        }
    } 
    9999
}

// "incrementing" an array of elements between 1 and max.
//
// For example, with max = 3 we will have
//
// [1,1,1] + 1 = [1,1,2]
// [1,2,3] + 1 = [1,3,1]
// [3,3,3] + 1 = [1,1,1] <- note that we're cycling back


fn increment_strat(mut v: Vec<u8>, max: u8) -> Vec<u8> {
    // last index
    let mut idx = v.len() - 1;
    
    // if the last index is strictly less than max, 
    // we increment it and we're done
    if v[idx] < max {
        v[idx] = v[idx] + 1;
        return v;
    } else {
        v[idx] = 1;
    }
    
    // 
    let mut done = false;
    while !done && idx != 0 {
        idx = idx - 1;
        if v[idx] == max {
            v[idx] = 1;
        } else {
            v[idx] = v[idx] + 1;
            done = true
        }
    }
    v
}


///////////////////////////////////////////////////////////////////////////////

// Tests
//
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_path_generation_3_2() {
        let ref_paths = get_paths(3, 2);
        
        let mut paths: Vec<WPath> = vec![];
        paths.push(WPath::new(vec![1, 2], 1.0/3.0));
        paths.push(WPath::new(vec![2, 1], 1.0/6.0));
        paths.push(WPath::new(vec![2, 3], 1.0/6.0));
        paths.push(WPath::new(vec![3, 2], 1.0/3.0));
        
        assert_eq!(ref_paths, paths);
    }

    #[test]
    fn check_path_generation_3_3() {
        let ref_paths = get_paths(3, 3);
        
        let mut paths: Vec<WPath> = vec![];
        paths.push(WPath::new(vec![1, 2, 1], 1.0/6.0));
        paths.push(WPath::new(vec![1, 2, 3], 1.0/6.0));
        paths.push(WPath::new(vec![2, 1, 2], 1.0/6.0));
        paths.push(WPath::new(vec![2, 3, 2], 1.0/6.0));
        paths.push(WPath::new(vec![3, 2, 1], 1.0/6.0));
        paths.push(WPath::new(vec![3, 2, 3], 1.0/6.0));
        
        assert_eq!(ref_paths, paths);
    }

}