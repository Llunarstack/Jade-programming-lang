# Utility functions for J

fn | square(int | x) > x * x

fn | greet(str | name) > "Hello, " + name + "!"

fn | factorial(int | n) > {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

# Test the functions
out("Testing utility functions:")
out(square(5))
out(greet("J Language"))
out(factorial(5))