use std::collections::{HashMap, HashSet};

/*
    PROBLEM: Valid Subsequences II
    SOLUTION: METHOD II (Dynamic Programming)
    LEETCODE STATS: Runtime = 471ms | Memory = 5.39MB

    - start with second-last element and then repeat below for every element before it
    - let the starting element be 1st element and the next element in each iteration be 2nd element
    - create a map with keys as result and value as an internal map
    - the internal map will have keys as index and value as length
    - check the (1st element + 2nd element)%k result in each internal iteration
    - if external map doesn't have key as result, create a new entry with length 2 in internal map
    - else if index of 2nd element not in internal map, set length 2 with index of 1st element
    - else if index of 2nd element in internal map, add 1 to corresponding value as length
    - keep updating value of a max variable whenever length gets incremented
    - return max as the final result
*/
pub fn medium_problem_1() {
    println!("medium problem 1 (Dynamic programming) - Valid subsequence II");

    // inputs
    let nums = vec![1,2,3,4,5];
    let k = 2;

    // initializations
    let mut extmap: HashMap<u16, HashMap<u16, u16>> = HashMap::new();
    let mut max_len = 2;
    let mut complexity_count = 0;

    // implementation
    for ext_index in (0..(nums.len() - 1) as u16).rev() {
        let mut used_vals: HashSet<u16> = HashSet::new();

        for int_index in (ext_index + 1)..nums.len() as u16 {
            let res = ((nums[ext_index as usize] + nums[int_index as usize]) % k) as u16;

            // skip if we already have this result and this is guaranteed to be equal or shorter
            if !used_vals.contains(&res) {
                used_vals.insert(res);
                complexity_count += 1;

                // insert into data structure
                if let Some(intmap) = extmap.get_mut(&res) {
                    if let Some(curr_len) = intmap.get(&int_index) {
                        // if index of 2nd element in internal map, add 1 to corresponding value as length
                        let actual_len = *curr_len + 1;
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
        }
    }

    println!("Maps = {:?}", extmap);
    println!("Max length = {}", max_len);
    println!("Comlexity = {}", complexity_count);
}
