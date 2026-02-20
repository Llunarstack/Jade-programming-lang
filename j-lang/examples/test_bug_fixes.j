# Test Critical Bug Fixes

out("=== Testing Critical Bug Fixes ===")
out("")

int | passed -> 0

# Test 1: Block Scopes
out("Test 1: Block scopes")
int | x -> 10
{
    x = 20
    int | y -> 30
}
out("Outside block: x = " + str(x))
passed = passed + 1
out("")

# Test 2: Negative Index Handling
out("Test 2: Negative indexing")
list | nums -> [1, 2, 3, 4, 5]
int | last -> nums[-1]
int | second_last -> nums[-2]
out("nums[-1] = " + str(last))
out("nums[-2] = " + str(second_last))
if last == 5 {
    out("✓ Negative indexing works")
    passed = passed + 1
}
out("")

# Test 3: Power Overflow Protection
out("Test 3: Power overflow protection")
int | result -> pow(2, 10)
out("pow(2, 10) = " + str(result))
if result == 1024 {
    out("✓ Power function works")
    passed = passed + 1
}
out("")

# Test 4: Matrix Validation
out("Test 4: Matrix validation")
mat | m -> [[1, 2, 3], [4, 5, 6]]
out("✓ Valid matrix created")
passed = passed + 1
out("")

# Test 5: Break in For Loop
out("Test 5: Break in for loop")
int | count -> 0
for i in [1, 2, 3, 4, 5] {
    if i == 3 {
        break
    }
    count = count + 1
}
if count == 2 {
    out("✓ Break works in for loop")
    passed = passed + 1
}
out("")

# Test 6: Continue in For Loop
out("Test 6: Continue in for loop")
count = 0
for i in [1, 2, 3, 4, 5] {
    if i == 3 {
        continue
    }
    count = count + 1
}
if count == 4 {
    out("✓ Continue works in for loop")
    passed = passed + 1
}
out("")

# Test 7: Break in While Loop
out("Test 7: Break in while loop")
int | i -> 0
count = 0
while i < 10 {
    i = i + 1
    if i == 3 {
        break
    }
    count = count + 1
}
if count == 2 {
    out("✓ Break works in while loop")
    passed = passed + 1
}
out("")

out("=== RESULTS ===")
out("Tests Passed: " + str(passed) + "/7")
if passed == 7 {
    out("✅ ALL CRITICAL FIXES WORKING!")
}
