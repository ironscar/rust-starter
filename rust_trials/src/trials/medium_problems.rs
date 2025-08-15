pub fn medium_problem_1() {
    println!("medium problem 1 (Dynamic programming) - Valid subsequence II");

    // create new thread to get around main thread stack size issues
    std::thread::spawn(|| {
        // inputs
        let nums = vec![1,2,3,4,5];
        let k = 2;

        // initializations
        let mut extmap = [[0u16;1000];1000];
        let mut max_len = 2;
        let mut complexity_count = 0;

        // implementation
        for ext_index in (0..nums.len() - 1).rev() {
            let mut used_vals = [0u16;1000];

            for int_index in (ext_index + 1)..nums.len() {
                let res = ((nums[ext_index] + nums[int_index]) % k) as usize;

                // skip if we already have this result and this is guaranteed to be equal or shorter
                if used_vals[res] == 1 {
                    continue;
                }
                used_vals[res] = 1;
                complexity_count += 1;

                // insert into data structure
                if extmap[res][int_index] != 0 {
                    // if index of 2nd element in internal map, add 1 to corresponding value as length
                    extmap[res][ext_index] = extmap[res][int_index] + 1;
                    if extmap[res][ext_index] > max_len {
                        max_len = extmap[res][ext_index];
                    }
                } else {
                    // if index of 2nd element not in internal map, set length 2 with index of 1st element
                    extmap[res][ext_index] = 2;
                }
            }
        }

        // results
        println!("Max length = {}", max_len);
        println!("Comlexity = {}", complexity_count);
    }).join().unwrap();
}
