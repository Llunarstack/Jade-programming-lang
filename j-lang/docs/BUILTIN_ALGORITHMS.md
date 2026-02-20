# Built-in Algorithm Functions for J Language

All functions are built into the interpreter (no imports needed). These cover common Big Tech/LeetCode patterns.

## Math & Number Theory

### `gcd(a, b)` - Greatest Common Divisor
```j
out(gcd(48, 18))  # 6
```

### `lcm(a, b)` - Least Common Multiple
```j
out(lcm(12, 18))  # 36
```

### `is_prime(n)` - Prime Check
```j
out(is_prime(17))  # true
out(is_prime(18))  # false
```

### `factorial(n)` - Factorial (n!)
```j
out(factorial(5))  # 120
out(factorial(10))  # 3628800
```

### `fibonacci(n)` - Nth Fibonacci Number
```j
out(fibonacci(10))  # 55
out(fibonacci(15))  # 610
```

## List Manipulation

### `swap(list, i, j)` - Swap Elements
```j
list | arr -> [1, 2, 3, 4, 5]
out(swap(arr, 1, 3))  # [1, 4, 3, 2, 5]
```

### `rotate_left(list, k)` - Rotate Left
```j
list | nums -> [1, 2, 3, 4, 5]
out(rotate_left(nums, 2))  # [3, 4, 5, 1, 2]
```

### `rotate_right(list, k)` - Rotate Right
```j
list | nums -> [1, 2, 3, 4, 5]
out(rotate_right(nums, 2))  # [4, 5, 1, 2, 3]
```

## Predicate Functions

### `count_if(list, predicate)` - Count Matching Elements
```j
list | nums -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
int | even_count -> count_if(nums, |x| x % 2 == 0)
out(even_count)  # 5
```

### `find_index(list, predicate)` - Find First Match
```j
list | data -> [10, 20, 30, 40, 50]
int | idx -> find_index(data, |x| x > 25)
out(idx)  # 2
```

## LeetCode Patterns

### `prefix_sum(list)` - Prefix Sum Array
**Pattern**: Range queries in O(1)
```j
list | nums -> [1, 2, 3, 4, 5]
list | prefix -> prefix_sum(nums)
out(prefix)  # [0, 1, 3, 6, 10, 15]
```

### `range_sum(prefix, i, j)` - Range Sum Query
**Pattern**: Sum from index i to j using prefix sum
```j
int | sum -> range_sum(prefix, 1, 3)  # Sum of elements 1-3
out(sum)  # 9 (2+3+4)
```

### `two_sum_indices(list, target)` - Two Sum
**Pattern**: Hash map for O(n) solution
```j
list | arr -> [2, 7, 11, 15]
list | indices -> two_sum_indices(arr, 9)
out(indices)  # [0, 1]
```

### `three_sum(list, target)` - Three Sum
**Pattern**: Two pointers after sorting
```j
list | nums -> [-1, 0, 1, 2, -1, -4]
list | triplets -> three_sum(nums, 0)
out(triplets)  # [[-1, -1, 2], [-1, 0, 1]]
```

### `max_sliding_window(list, k)` - Sliding Window Maximum
**Pattern**: Monotonic deque (simplified implementation)
```j
list | arr -> [1, 3, -1, -3, 5, 3, 6, 7]
list | max_vals -> max_sliding_window(arr, 3)
out(max_vals)  # [3, 3, 5, 5, 6, 7]
```

### `longest_increasing_subsequence(list)` - LIS Length
**Pattern**: Dynamic programming O(n²)
```j
list | arr -> [10, 9, 2, 5, 3, 7, 101, 18]
int | length -> longest_increasing_subsequence(arr)
out(length)  # 4
```

### `merge_intervals(intervals)` - Merge Overlapping Intervals
**Pattern**: Sorting + greedy
```j
list | int1 -> [1, 3]
list | int2 -> [2, 6]
list | int3 -> [8, 10]
list | int4 -> [15, 18]
list | intervals -> [int1, int2, int3, int4]
list | merged -> merge_intervals(intervals)
out(merged)  # [[1, 6], [8, 10], [15, 18]]
```

### `topological_sort(graph)` - Topological Sort
**Pattern**: Kahn's algorithm (BFS with in-degrees)
```j
dict | graph -> {
    A: ["B", "C"],
    B: ["D"],
    C: ["D"],
    D: []
}
list | order -> topological_sort(graph)
out(order)  # ["A", "B", "C", "D"] or ["A", "C", "B", "D"]
```

### `dijkstra(graph, start)` - Shortest Path
**Pattern**: Dijkstra's algorithm (already existed)
```j
graph | g -> {}
g = add_node(g, "A")
g = add_node(g, "B")
g = add_edge(g, "A", "B", 5)
dict | result -> dijkstra(g, "A")
out(result)  # {path: [...], distance: ...}
```

## Usage Examples

### Example 1: Range Sum Queries
```j
list | nums -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
list | prefix -> prefix_sum(nums)

# Query sum from index 2 to 5
int | sum -> range_sum(prefix, 2, 5)
out(sum)  # 18 (3+4+5+6)
```

### Example 2: Finding Pairs
```j
list | arr -> [3, 2, 4]
list | result -> two_sum_indices(arr, 6)
out(result)  # [1, 2] (indices of 2 and 4)
```

### Example 3: Sliding Window
```j
list | prices -> [7, 1, 5, 3, 6, 4]
list | max_in_window -> max_sliding_window(prices, 3)
out(max_in_window)  # [7, 5, 6, 6]
```

## Performance Notes

- `prefix_sum`: O(n) time, O(n) space
- `range_sum`: O(1) time
- `two_sum_indices`: O(n) time, O(n) space (hash map)
- `three_sum`: O(n²) time, O(1) space
- `max_sliding_window`: O(nk) time (can be optimized to O(n) with deque)
- `longest_increasing_subsequence`: O(n²) time, O(n) space
- `merge_intervals`: O(n log n) time (sorting)
- `topological_sort`: O(V + E) time
- `dijkstra`: O(V²) time (can be optimized with priority queue)

## Tips for LeetCode

1. **Use prefix sums** for range query problems
2. **Use hash maps** (two_sum pattern) for O(n) lookups
3. **Use two pointers** after sorting for pair/triplet problems
4. **Use sliding window** for subarray problems
5. **Use DP** (LIS pattern) for subsequence problems
6. **Use sorting + greedy** for interval problems
7. **Use topological sort** for dependency/course schedule problems
8. **Use Dijkstra** for shortest path in weighted graphs

All functions work with simple, readable syntax - no complex data structure declarations needed!
