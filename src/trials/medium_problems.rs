use std::collections::HashMap;

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
