# Test generator functionality

# Simple generator function
fn | countdown (int | n) > {
    int | i -> n
    while i >= 0 {
        yield i
        i = i - 1
    }
}

# Test the generator
out("Testing generator:")
i in countdown(5) : out(i)

# Fibonacci generator
fn | fib () > {
    int | a -> 0
    int | b -> 1
    while true {
        yield a
        int | temp -> a
        a = b
        b = temp + b
    }
}

out("\nFirst 10 Fibonacci numbers:")
int | count -> 0
i in fib() {
    if count >= 10 { break }
    out(i)
    count = count + 1
}
