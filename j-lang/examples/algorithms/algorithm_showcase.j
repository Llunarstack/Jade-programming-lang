# ========================================
# ALGORITHM SHOWCASE
# Real-world algorithm implementations
# using advanced loop constructs
# ========================================

out("========================================")
out("ALGORITHM SHOWCASE")
out("========================================\n")

# ========================================
# 1. Longest Substring Without Repeating Characters
# ========================================
out("=== Algorithm 1: Longest Unique Substring ===")

list | chars -> [1, 2, 3, 2, 4, 5, 3, 6]
int | max_unique -> 0

sweep (start, end) in chars {
    # Check if current window has duplicates
    list | window -> chars[start .. end + 1]
    int | unique_count -> len(unique(window))
    int | window_size -> end - start + 1
    
    if unique_count == window_size {
        if window_size > max_unique {
            max_unique = window_size
        }
        end = end + 1
    } else {
        start = start + 1
    }
}

out("Input:", chars)
out("Longest unique substring length:", max_unique)
out()

# ========================================
# 2. Container With Most Water
# ========================================
out("=== Algorithm 2: Container With Most Water ===")

list | heights -> [1, 8, 6, 2, 5, 4, 8, 3, 7]
int | max_area -> 0

meet (left, right) in heights {
    int | width -> right - left
    int | height -> min(heights[left], heights[right])
    int | area -> width * height
    
    if area > max_area {
        max_area = area
    }
    
    if heights[left] < heights[right] {
        left = left + 1
    } else {
        right = right - 1
    }
}

out("Heights:", heights)
out("Maximum water area:", max_area)
out()

# ========================================
# 3. Binary Search - Find First Occurrence
# ========================================
out("=== Algorithm 3: Find First Occurrence ===")

list | sorted_arr -> [1, 2, 2, 2, 3, 4, 5, 5, 5, 6]
int | find_target -> 5
int | first_pos -> -1

binary (low, high) in sorted_arr {
    int | middle -> (low + high) / 2
    
    if sorted_arr[middle] == find_target {
        first_pos = middle
        high = middle - 1  # Continue searching left
    } else {
        if sorted_arr[middle] < find_target {
            low = middle + 1
        } else {
            high = middle - 1
        }
    }
}

out("Array:", sorted_arr)
out("Target:", find_target)
out("First occurrence at index:", first_pos)
out()

# ========================================
# 4. Hamming Distance (Bit Difference)
# ========================================
out("=== Algorithm 4: Hamming Distance ===")

int | num1 -> 42
int | num2 -> 27
int | xor_result -> num1 ^ num2
int | hamming_dist -> 0

while_nonzero xor_result {
    hamming_dist = hamming_dist + (xor_result & 1)
    xor_result = xor_result >> 1
}

out("Number 1:", num1)
out("Number 2:", num2)
out("Hamming distance:", hamming_dist)
out()

# ========================================
# 5. Square Root (Newton's Method)
# ========================================
out("=== Algorithm 5: Square Root Approximation ===")

int | sqrt_iterations -> 0

while_change approx = 50 {
    int | target_num -> 100
    approx = (approx + target_num / approx) / 2
    sqrt_iterations = sqrt_iterations + 1
}

out("Square root of 100 ≈", approx)
out("Converged in", sqrt_iterations, "iterations")
out()

# ========================================
# 6. Valid Palindrome
# ========================================
out("=== Algorithm 6: Valid Palindrome ===")

list | palindrome_test -> [1, 2, 3, 2, 1]
bool | is_palindrome -> true

meet (p_left, p_right) in palindrome_test {
    if palindrome_test[p_left] != palindrome_test[p_right] {
        is_palindrome = false
        p_left = p_right  # Exit
    } else {
        p_left = p_left + 1
        p_right = p_right - 1
    }
}

out("Array:", palindrome_test)
out("Is palindrome?", is_palindrome)
out()

# ========================================
# 7. Count Set Bits in Range
# ========================================
out("=== Algorithm 7: Count Set Bits in Range ===")

list | bit_numbers -> [5, 7, 12, 15, 31]
int | total_bits -> 0

i in bit_numbers {
    int | temp -> i
    while_nonzero temp {
        total_bits = total_bits + (temp & 1)
        temp = temp >> 1
    }
}

out("Numbers:", bit_numbers)
out("Total set bits:", total_bits)
out()

# ========================================
# 8. Maximum Subarray Sum (Fixed Size)
# ========================================
out("=== Algorithm 8: Max Subarray Sum (Size 3) ===")

list | numbers -> [2, 1, 5, 1, 3, 2, 7, 4]
int | window_size -> 3
int | max_sum -> 0
int | current_window_sum -> 0

sweep (ws, we) in numbers {
    if we - ws + 1 == window_size {
        current_window_sum = sum(numbers[ws .. we + 1])
        if current_window_sum > max_sum {
            max_sum = current_window_sum
        }
        ws = ws + 1
    } else {
        we = we + 1
    }
}

out("Array:", numbers)
out("Window size:", window_size)
out("Maximum sum:", max_sum)
out()

# ========================================
# 9. Power of Two Detection
# ========================================
out("=== Algorithm 9: Power of Two Detection ===")

list | test_numbers -> [1, 2, 3, 4, 5, 8, 15, 16, 32, 33]

out("Testing numbers:", test_numbers)
i in test_numbers {
    bool | is_power_of_two -> i > 0 and (i & (i - 1)) == 0
    out(i, "is power of 2:", is_power_of_two)
}
out()

# ========================================
# 10. Bitwise Swap Without Temp
# ========================================
out("=== Algorithm 10: XOR Swap ===")

int | swap_a -> 42
int | swap_b -> 17

out("Before swap: a =", swap_a, ", b =", swap_b)

swap_a = swap_a ^ swap_b
swap_b = swap_a ^ swap_b
swap_a = swap_a ^ swap_b

out("After swap:  a =", swap_a, ", b =", swap_b)
out()

out("========================================")
out("ALL ALGORITHMS COMPLETE!")
out("========================================")
out("\n✓ 10 real-world algorithms implemented")
out("✓ Using advanced loop constructs")
out("✓ Demonstrating practical patterns")
