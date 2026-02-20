# Built-in Algorithm Functions Demo

out("========================================")
out("BUILT-IN ALGORITHM FUNCTIONS")
out("========================================\n")

# GCD and LCM
out("1. GCD and LCM")
out("GCD(48, 18):", gcd(48, 18))
out("LCM(12, 18):", lcm(12, 18))
out("")

# Prime Check
out("2. Prime Check")
out("is_prime(17):", is_prime(17))
out("is_prime(18):", is_prime(18))
out("is_prime(97):", is_prime(97))
out("")

# Factorial
out("3. Factorial")
out("factorial(5):", factorial(5))
out("factorial(10):", factorial(10))
out("")

# Fibonacci
out("4. Fibonacci")
out("fibonacci(10):", fibonacci(10))
out("fibonacci(15):", fibonacci(15))
out("")

# Swap
out("5. Swap Elements")
list | arr -> [1, 2, 3, 4, 5]
out("Original:", arr)
out("After swap(arr, 1, 3):", swap(arr, 1, 3))
out("")

# Rotate
out("6. Rotate List")
list | nums -> [1, 2, 3, 4, 5]
out("Original:", nums)
out("Rotate left by 2:", rotate_left(nums, 2))
out("Rotate right by 2:", rotate_right(nums, 2))
out("")

# Count If
out("7. Count If")
list | numbers -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
int | even_count -> count_if(numbers, |x| x % 2 == 0)
out("Numbers:", numbers)
out("Even count:", even_count)
out("")

# Find Index
out("8. Find Index")
list | data -> [10, 20, 30, 40, 50]
int | idx -> find_index(data, |x| x > 25)
out("Data:", data)
out("First index where x > 25:", idx)
out("")

out("========================================")
out("ALL BUILT-IN FUNCTIONS DEMONSTRATED!")
out("========================================")
