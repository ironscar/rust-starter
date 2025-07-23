use std::collections::{HashMap, HashSet};

/*
    PROBLEM: Valid Subsequences II
    SOLUTION: METHOD II (Dynamic Programming)

    start with second-last element and then repeat below for every element before it
    let the starting element be 1st element and the next element in each iteration be 2nd element
    create a map with keys as result and value as an internal map
    the internal map will have keys as index and value as length
    check the (1st element + 2nd element)%k result in each internal iteration
    if external map doesn't have key as result, create a new entry with length 2 in internal map
    else if index of 2nd element not in internal map, set length 2 with index of 1st element
    else if index of 2nd element in internal map, add 1 to corresponding value as length
    keep updating value of a max variable whenever length gets incremented
    return max as the final result

    Leetcode stats:
    Runtime = 716ms and Memory = 13.09MB
*/
pub fn medium_problem_1() {
    println!("medium problem 1 (Dynamic programming) - Valid subsequence II");

    // inputs
    let nums = vec![4,5,10,2,4,9];
    let k = 2;

    // initializations
    let mut ext_index = nums.len() - 2;
    let mut extmap: HashMap<i32, HashMap<usize, usize>> = HashMap::new();
    let mut max_len = 2;
    let mut complexity_count = 0;

    // implementation
    loop {
        let mut int_index = ext_index + 1;
        let mut used_vals: HashSet<i32> = HashSet::new();

        loop {
            let res = (nums[ext_index] + nums[int_index]) % k;

            // skip if we already have this result and this is guaranteed to be equal or shorter
            if !used_vals.contains(&res) {
                used_vals.insert(res);
                complexity_count += 1;

                // insert into data structure
                if let Some(intmap) = extmap.get_mut(&res) {
                    if let Some(curr_len) = intmap.get(&int_index) {
                        // if index of 2nd element in internal map, add 1 to corresponding value as length
                        let actual_len = (*curr_len).clone() + 1;
                        intmap.insert(ext_index, actual_len);
                        if actual_len > max_len {
                            max_len = actual_len;
                        }
                    } else {
                        // if index of 2nd element not in internal map, set length 2 with index of 1st element
                        intmap.insert(ext_index, 2);
                    }
                } else {
                    // if external map doesn't have key as result, create a new entry with length 2
                    let mut int_map1 = HashMap::new();
                    int_map1.insert(ext_index, 2);
                    extmap.insert(res, int_map1);
                }
            }

            // increment int_index
            int_index += 1;
            if int_index == nums.len() {
                break;
            }
        }

        // decrement ext_index
        if ext_index == 0 {
            break;
        }
        ext_index -= 1;
    }

    println!("Maps = {:?}", extmap);
    println!("Max length = {}", max_len);
    println!("Comlexity = {}", complexity_count);
}
