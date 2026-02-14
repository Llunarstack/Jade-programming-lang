# Utility functions for J

str | utils_loaded -> "Utils module loaded!"
out(utils_loaded)

# Simple utility functions
fn int | double (int | x) > x * 2

fn int | triple (int | x) > x * 3

fn bool | is_even (int | x) > x % 2 == 0

fn int | factorial (int | n) > {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

# Test the functions
out("Testing utility functions:")
out("Double of 5: " + double(5))
out("Triple of 4: " + triple(4))
out("Is 6 even? " + is_even(6))
out("Is 7 even? " + is_even(7))
out("Factorial of 5: " + factorial(5))