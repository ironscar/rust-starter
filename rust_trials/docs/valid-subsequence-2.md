# Valid Subsequences II

## LEETCODE STATS

- Runtime = 42ms
- Memory = 4.1MB

## SOLUTION

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

## OPTIMIZATIONS

- Using u16 instead of i32 wherever possible (memory: 13MB -> 5.3MB, runtime: 0.7s -> 0.45s)
- Using a fixed-size array instead of HashSet (memory: 5.3MB -> 5.5MB, runtime: 0.45s -> 0.3s)
- Using a fixed-size array instead of HashMap (memory: 5.5MB -> 4.12MB, runtime: 0.3s -> 0.04s)
  - this causes the main thread to go out of stack on local machine for k = 1000 & n = 1000
  - creating new thread to do the computations instead which tends to work out locally
