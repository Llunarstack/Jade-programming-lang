# Test newly added features

# Test 1: Counter type
out("=== Testing Counter ===")
counter | c1 -> counter("hello")
out(c1)
out("Most common: ")
out(c1.most_common())

# Test 2: Grid type
out("")
out("=== Testing Grid ===")
grid | g -> [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
out("Grid:")
out(g)
out("Rows: " + str(g.rows))
out("Cols: " + str(g.cols))

# Test 3: Defer statement
out("")
out("=== Testing Defer ===")
{
    out("Start of block")
    defer out("This runs at end of block")
    out("Middle of block")
}
out("After block")

# Test 4: Class with this keyword
out("")
out("=== Testing Class with this ===")
class | Calculator {
    int | value -> 0
    
    fn | add(int | n) > {
        value = value + n
        out("Added " + str(n) + ", value is now " + str(value))
    }
}

calc -> Calculator.new()
calc.add(5)
calc.add(10)
out("Final value: " + str(calc.value))
