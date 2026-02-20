# LeetCode Algorithm Patterns - Built-in Functions

out("========================================")
out("LEETCODE ALGORITHM PATTERNS")
out("========================================\n")

# 1. PREFIX SUM - Range Query Pattern
out("1. PREFIX SUM (Range Queries)")
list | nums -> [1, 2, 3, 4, 5]
list | prefix -> prefix_sum(nums)
out("Array:", nums)
out("Prefix sum:", prefix)
out("Sum from index 1 to 3:", range_sum(prefix, 1, 3))
out("")

# 2. TWO SUM - Hash Map Pattern
out("2. TWO SUM (Hash Map)")
list | arr -> [2, 7, 11, 15]
int | target -> 9
list | indices -> two_sum_indices(arr, target)
out("Array:", arr, "Target:", target)
out("Indices:", indices)
out("")

# 3. THREE SUM - Two Pointers Pattern
out("3. THREE SUM (Two Pointers)")
list | nums3 -> [-1, 0, 1, 2, -1, -4]
list | triplets -> three_sum(nums3, 0)
out("Array:", nums3)
out("Triplets that sum to 0:", triplets)
out("")

# 4. SLIDING WINDOW - Maximum in Window
out("4. SLIDING WINDOW (Maximum)")
list | window_arr -> [1, 3, -1, -3, 5, 3, 6, 7]
list | max_vals -> max_sliding_window(window_arr, 3)
out("Array:", window_arr)
out("Max in each window of size 3:", max_vals)
out("")

# 5. LONGEST INCREASING SUBSEQUENCE - DP Pattern
out("5. LONGEST INCREASING SUBSEQUENCE (DP)")
list | lis_arr -> [10, 9, 2, 5, 3, 7, 101, 18]
int | lis_length -> longest_increasing_subsequence(lis_arr)
out("Array:", lis_arr)
out("LIS length:", lis_length)
out("")

# 6. MERGE INTERVALS - Sorting Pattern
out("6. MERGE INTERVALS (Sorting)")
list | intervals -> [[1, 3], [2, 6], [8, 10], [15, 18]]
list | merged -> merge_intervals(intervals)
out("Intervals:", intervals)
out("Merged:", merged)
out("")

# 7. TOPOLOGICAL SORT - Graph Pattern
out("7. TOPOLOGICAL SORT (Graph)")
dict | graph -> { 
    A: ["B", "C"],
    B: ["D"],
    C: ["D"],
    D: []
}
list | topo_order -> topological_sort(graph)
out("Graph adjacency list:", graph)
out("Topological order:", topo_order)
out("")

# 8. BASIC MATH FUNCTIONS
out("8. MATH UTILITIES")
out("GCD(48, 18):", gcd(48, 18))
out("LCM(12, 18):", lcm(12, 18))
out("is_prime(17):", is_prime(17))
out("factorial(6):", factorial(6))
out("fibonacci(10):", fibonacci(10))
out("")

# 9. LIST MANIPULATION
out("9. LIST MANIPULATION")
list | data -> [1, 2, 3, 4, 5]
out("Original:", data)
out("Swap(1, 3):", swap(data, 1, 3))
out("Rotate left 2:", rotate_left(data, 2))
out("Rotate right 2:", rotate_right(data, 2))
out("")

# 10. PREDICATES
out("10. PREDICATE FUNCTIONS")
list | numbers -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
out("Numbers:", numbers)
out("Count even:", count_if(numbers, |x| x % 2 == 0))
out("First > 5:", find_index(numbers, |x| x > 5))
out("")

out("========================================")
out("ALL PATTERNS DEMONSTRATED!")
out("========================================")
