# Advanced Algorithm Loop Constructs

This document describes the 8 advanced loop constructs added to the J programming language, designed specifically for common algorithm patterns.

## Overview

These loops provide syntactic sugar for common algorithmic patterns, making code more readable and intention-revealing:

1. **sweep** - Two-pointer sliding window
2. **shrink** - Trim from both ends
3. **meet** - Opposite-direction pointers
4. **binary** - Binary search pattern
5. **dp** - Dynamic programming table initialization
6. **while_nonzero** - Bit manipulation loops
7. **while_change** - Convergence loops
8. **while_match** - Pattern matching loops

## 1. Sweep Loop

**Syntax:** `sweep (left, right) in collection { ... }`

**Purpose:** Two-pointer sliding window pattern for subarray/substring problems.

**Behavior:**
- Initializes `left = 0` and `right = 0`
- User controls pointer movement inside the loop body
- Loop continues until `right >= length`

**Example:**
```j
list | nums -> [1, 2, 3, 4, 5]
int | target -> 9
int | max_len -> 0

sweep (left, right) in nums {
    int | window_sum -> sum(nums[left .. right + 1])
    
    if window_sum <= target {
        if right - left + 1 > max_len {
            max_len = right - left + 1
        }
        right = right + 1
    } else {
        left = left + 1
    }
}
```

## 2. Shrink Loop

**Syntax:** `shrink (left, right) in collection { ... }`

**Purpose:** Process array from both ends, trimming inward.

**Behavior:**
- Initializes `left = 0` and `right = length - 1`
- Loop continues while `left <= right`
- User updates pointers in loop body

**Example:**
```j
list | arr -> [1, 2, 3, 4, 5]

shrink (left, right) in arr {
    out("Processing:", arr[left], arr[right])
    left = left + 1
}
```

## 3. Meet Loop

**Syntax:** `meet (left, right) in collection { ... }`

**Purpose:** Two pointers moving toward each other (two-sum pattern).

**Behavior:**
- Initializes `left = 0` and `right = length - 1`
- Loop continues while `left < right`
- User updates pointers based on comparison

**Example:**
```j
list | sorted -> [1, 2, 3, 4, 5, 6, 7, 8, 9]
int | target -> 10

meet (l, r) in sorted {
    int | sum -> sorted[l] + sorted[r]
    
    if sum == target {
        out("Found:", sorted[l], "+", sorted[r])
        l = r  # Exit loop
    } else {
        if sum < target {
            l = l + 1
        } else {
            r = r - 1
        }
    }
}
```

## 4. Binary Search Loop

**Syntax:** `binary (lo, hi) in collection { ... } else { ... }`

**Purpose:** Binary search pattern with optional else block for "not found".

**Behavior:**
- Initializes `lo = 0` and `hi = length - 1`
- Loop continues while `lo <= hi`
- User calculates mid and updates bounds
- Optional else block executes if not found

**Example:**
```j
list | arr -> [1, 3, 5, 7, 9, 11, 13, 15]
int | target -> 11
int | result -> -1

binary (lo, hi) in arr {
    int | mid -> (lo + hi) / 2
    
    if arr[mid] == target {
        result = mid
        lo = lo  # Keep same to exit
        hi = hi
    } else {
        if arr[mid] < target {
            lo = mid + 1
        } else {
            hi = mid - 1
        }
    }
} else {
    out("Not found")
}
```

## 5. DP Loop

**Syntax:** `dp table_name[dim1][dim2]... = init_value { ... }`

**Purpose:** Initialize dynamic programming tables.

**Behavior:**
- Creates a table with specified dimensions
- Initializes all cells to `init_value`
- Table is available as a variable in the body
- Supports 1D and 2D tables

**Example:**
```j
# 1D table
dp cache[10] = 0 {
    out("Cache initialized")
}

# 2D table
dp grid[5][5] = 1 {
    out("Grid initialized")
}
```

**Note:** Currently, indexed assignment (`table[i] = value`) is not yet implemented. Tables can be initialized but individual cell updates require workarounds.

## 6. While Nonzero Loop

**Syntax:** `while_nonzero var { ... }`

**Purpose:** Loop while a variable is non-zero (common in bit manipulation).

**Behavior:**
- Checks if `var != 0` before each iteration
- Exits when `var == 0`
- User must update `var` in loop body

**Example:**
```j
int | num -> 42
int | count -> 0

while_nonzero num {
    count = count + (num & 1)
    num = num >> 1
}

out("Bit count:", count)  # Output: 3
```

## 7. While Change Loop

**Syntax:** `while_change var = init_value { ... }`

**Purpose:** Loop until a value stabilizes (convergence pattern).

**Behavior:**
- Initializes `var` to `init_value`
- Executes body
- Continues if `var` changed during iteration
- Exits when `var` remains unchanged

**Example:**
```j
int | iterations -> 0

while_change val = 100 {
    val = val / 2
    iterations = iterations + 1
}

out("Converged after", iterations, "iterations")
```

## 8. While Match Loop

**Syntax:** `while_match var { ... }`

**Purpose:** Pattern-matching while loop.

**Behavior:**
- Continues while `var` is not `None`
- Exits when `var == None`
- Useful for optional value processing

**Example:**
```j
# Currently implemented as checking for None value
# User must set var to None to exit
```

## Bitwise Operations

These loops work seamlessly with bitwise operations:

- `&` - Bitwise AND
- `|` - Bitwise OR
- `^` - Bitwise XOR
- `~` - Bitwise NOT
- `<<` - Left shift
- `>>` - Right shift

**Example:**
```j
int | a -> 12
int | b -> 10

out("AND:", a & b)      # 8
out("OR:", a | b)       # 14
out("XOR:", a ^ b)      # 6
out("NOT:", ~a)         # -13
out("Left:", a << 2)    # 48
out("Right:", a >> 2)   # 3

# Power of 2 check
bool | is_pow2 -> n > 0 and (n & (n - 1)) == 0
```

## Implementation Status

✅ **Fully Implemented:**
- sweep loop
- shrink loop
- meet loop
- binary search loop
- dp loop (table initialization)
- while_nonzero loop
- while_change loop
- while_match loop
- All bitwise operators

⚠️ **Limitations:**
- Indexed assignment (`table[i] = value`) not yet implemented
- Break/continue statements not fully integrated with new loops
- while_match currently checks for None value only

## Testing

Run the comprehensive test:
```bash
j run examples/ADVANCED_LOOPS_COMPLETE.j
```

Individual tests:
```bash
j run examples/test_sweep.j
j run examples/test_meet.j
j run examples/test_shrink.j
j run examples/test_binary.j
j run examples/test_dp.j
j run examples/test_while_nonzero.j
```

## Use Cases

**Sweep Loop:**
- Longest substring without repeating characters
- Maximum sum subarray with size constraint
- Sliding window maximum/minimum

**Shrink Loop:**
- Remove duplicates from sorted array
- Valid palindrome checking
- Container with most water

**Meet Loop:**
- Two sum in sorted array
- Three sum problem
- Trapping rain water

**Binary Search Loop:**
- Find element in sorted array
- Find insertion position
- Search in rotated sorted array

**DP Loop:**
- Fibonacci sequence
- Longest common subsequence
- Grid path counting

**While Nonzero:**
- Bit counting (Hamming weight)
- Power of 2 detection
- Bit manipulation algorithms

**While Change:**
- Iterative algorithms
- Fixed-point computation
- Convergence detection

## Future Enhancements

- Indexed assignment for DP tables
- Break/continue integration
- More pattern matching options for while_match
- 3D+ DP table support
- Automatic bounds checking
- Performance optimizations
