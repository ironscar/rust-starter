use std::collections::{HashMap, HashSet};
use std::collections::hash_map;
/*
    PROBLEM: Valid Subsequences II
    SOLUTION: METHOD I (Brute force complexity = 2^n)
    SUBMISSION: Failed due to memory limit

    for each element, find the value of the next elements sum mod k and push into 2-part list
    then start building 3-element combos from the 2-part list and matching indexes
    then start building 4-element combos using the 3-part and referencing it with 2-part
    keep going with using the prev element combos referenced with 2-part combos based on index
    no more iterations if only final index elements remaining in list so print results then
*/
pub fn medium_problem_1a() {
    println!("medium problem 1 (Brute force) - Valid subsequence II");

    // inputs
    let nums = vec![4,5,10,2,4,9];
    let k = 2;

    // definitions
    #[derive(Debug)]
    struct Subsequence {
        nums: Vec<i32>,
        result: i32,
        last_index: usize,
        is_final: bool
    }
    impl Subsequence {
        fn clone(&self) -> Self {
            Self {
                nums: self.nums.clone(),
                result: self.result,
                last_index: self.last_index,
                is_final: self.is_final
            }
        }
    }

    // initializations
    let mut v2: HashMap<usize, Vec<Subsequence>> = HashMap::new();
    let mut vprev: Vec<Subsequence> = Vec::new();
    let mut vn: Vec<Subsequence> = Vec::new();
    let mut curr_length = 1;
    let mut index1 = 0;
    let mut count_non_final = 0;
    let mut complexity_count = 0;

    // implementation
    loop {
        if vprev.is_empty() {
            // create the 2 element combos (elementary case)
            loop {
                let mut index2 = index1 + 1;
                let mut elems: Vec<Subsequence> = Vec::new();
                loop {
                    complexity_count += 1;
                    if index2 == nums.len() {
                        break;
                    }
                    let elem = Subsequence {
                        nums: vec![nums[index1], nums[index2]],
                        result: (nums[index1] + nums[index2]) % k,
                        last_index: index2,
                        is_final: index2 == nums.len() - 1
                    };
                    if !elem.is_final {
                        count_non_final += 1;
                    }
                    elems.push(elem.clone());
                    vprev.push(elem);
                    index2 += 1;
                }
                v2.insert(index1, elems);
                index1 += 1;
                if index1 == nums.len() - 1 {
                    break;
                }
            }
            curr_length += 1;
        }
        else if count_non_final != 0 {
            // create n-element combos by combining v2 with latest max subsequences (recursive case)
            index1 = 0;
            count_non_final = 0;
            if !vn.is_empty() && curr_length > 2 {
                vprev = vn;
                vn = Vec::new();
            }
            for prev_elem in &vprev {
                if prev_elem.is_final {
                    continue;
                }
                let end_index = prev_elem.last_index;
                let end_index_elems = v2.get(&end_index).unwrap();
                for end_elem in end_index_elems {
                    complexity_count += 1;
                    if end_elem.result == prev_elem.result {
                        // insert new element from here
                        let mut new_nums = prev_elem.nums.clone();
                        new_nums.push(nums[end_elem.last_index] as i32);
                        let new_elem = Subsequence {
                            nums: new_nums,
                            result: end_elem.result,
                            last_index: end_elem.last_index,
                            is_final: end_elem.is_final
                        };
                        if !new_elem.is_final {
                            count_non_final += 1;
                        }
                        vn.push(new_elem);
                    }
                }
            }
            if !vn.is_empty() {
                curr_length += 1;
            }
        }
        else if count_non_final == 0 {
            complexity_count += 1;
            // no more iterations required so check in vn if anything, else check in vprev
            if !vn.is_empty() {
                println!("Valid max subsequences of length {} = {:?}", curr_length, vn);
            }
            else if !vprev.is_empty() {
                println!("Valid max subsequences of length {} = {:?}", curr_length, vprev);
            }
            break;
        }
    }
    println!("Brute force complexity count = {} for size = {}", complexity_count, nums.len());
}

/*
    PROBLEM: Valid Subsequences II
    SOLUTION: METHOD II (Dynamic Programming)

    start with elementary case of last pair
    take [4,9] and k = 2 => 1, store in a map with key = 1
    value with internal map with key as index and set value as length 2 => [4 -> 2]
    ----
    add next element before elementary case to this
    take 2 and combine with 4 and 9 separately to see if we get 1
    [2,4], k = 2 => 0 => 0 -> [3 -> 2]
        - if external map doesn't have key as result, create a new entry with length 2
    [2,9], k = 2 => 1 => 1 -> [3 -> 2, 4 -> 2]
        - if index of 2nd element not in internal map, set length 2 with index of 1st element
    ----
    add next element before elementary case to this
    take 10 and combine with each element that comes after it
    [10,2], k = 2 => 0 => 0 -> [3 -> 3]
        - if index of 2nd element in internal map, add 1 to corresponding value as length
    [10,4], k = 2 => 0
        - skip because we already have this result and this is guaranteed to be equal or shorter
    [10,9], k = 2 => 1 => 1 -> [2 -> 2, 3 -> 2, 4 -> 2]
        - if index of 2nd element not in internal map, set length 2 with index of 1st element
    ----
    add next element before elementary case to this
    take 5 and combine with each element that comes after it
    [5,10], k = 2 => 1 => 1 -> [2 -> 3, 3 -> 2, 4 -> 2]
        - if index of 2nd element in internal map, add 1 to corresponding value as length
    [5,2], k = 2 => 1
        - skip because we already have this result and this is guaranteed to be equal or shorter
    [5,4], k = 2 => 1
        - skip because we already have this result and this is guaranteed to be equal or shorter
    [5,9], k = 2 => 0 => 0 -> [3 -> 3, 1 -> 2]
        - if index of 2nd element not in internal map, set length 2 with index of 1st element
    ----
    add next element before elementary case to this
    take 4 and combine with each element that comes after it
    [4,5], k = 2 => 1 => 1 -> [2 -> 4, 3 -> 2, 4 -> 2]
        - if index of 2nd element in internal map, add 1 to corresponding value as length
    [4,10], k = 2 => 0 => 0 -> [3 -> 4, 1 -> 2]
    [4,2], k = 2 => 0
        - skip because we already have this result and this is guaranteed to be equal or shorter
    [4,4], k = 2 => 0
        - skip because we already have this result and this is guaranteed to be equal or shorter
    [4,9], k = 2 => 1
        - skip because we already have this result and this is guaranteed to be equal or shorter
    ----
    once all elements are done, find the largest internal map value which will give length
    ----
    if k = 1000 and maxlength = 1000
    external map and internal map can have 1000 distinct keys => 10^6 max distinct values
*/
pub fn medium_problem_1b() {
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

// trial for updating internal map inside external map
pub fn map_trial() {
    let mut extmap: HashMap<i32, HashMap<usize, usize>> = HashMap::new();
    int_add1(&mut extmap);
    int_add2(&mut extmap);
    println!("Extmap = {:?}", extmap);
}

fn int_add2(extmap: &mut HashMap<i32, HashMap<usize, usize>>) {
    let mut intmap2 = extmap.get(&1).unwrap().clone();
    intmap2.insert(2, 2);
    extmap.insert(1, intmap2);
}

fn int_add1(extmap: &mut HashMap<i32, HashMap<usize, usize>>) {
    let mut intmap: HashMap<usize, usize> = HashMap::new();
    intmap.insert(1, 1);
    extmap.insert(1, intmap);
}
