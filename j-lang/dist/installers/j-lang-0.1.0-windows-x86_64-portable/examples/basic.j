# Basic J Language Examples

# Variable declarations with different types
str | greeting -> "Hello from J!"
int | answer -> 42
float | pi -> 3.14159
bool | is_awesome -> true
list | numbers -> [1, 2, 3, 4, 5]
tuple | point -> (10, 20)

# Print values
out(greeting)
out("The answer is: " + answer)
out("Pi is approximately: " + pi)
out("J is awesome: " + is_awesome)

# Work with lists
out("Numbers: " + numbers)
out("Sum of numbers: " + sum(numbers))
out("Length of numbers: " + len(numbers))

# Loops
out("Counting from 1 to 5:")
i in 1..6 : out(i)

out("Numbers doubled:")
in numbers : out(_ * 2)

# Indexed iteration
out("Numbers with indices:")
(i, v) in numbers : out("Index " + i + ": " + v)

# Functions
fn int | square (int | x) > x * x

out("Square of 7: " + square(7))

# Ranges
out("Range 0 to 10:")
r in range(11) : out(r)

# Pipeline operations
numbers |> map(square) |> out