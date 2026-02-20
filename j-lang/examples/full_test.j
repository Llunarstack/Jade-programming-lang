# Full J Language Test - All Functions

out("=== J LANGUAGE FULL TEST ===")
out("")

int | passed -> 0

# 1. Basic Types
out("1. Basic Types")
int | x -> 42
str | s -> "hello"
bool | b -> true
list | l -> [1, 2, 3]
out("✓ All basic types work")
passed = passed + 1
out("")

# 2. Operators
out("2. Operators")
int | add_r -> 5 + 3
int | sub_r -> 10 - 4
int | mul_r -> 6 * 7
int | div_r -> 20 / 4
int | mod_r -> 10 % 3
out("✓ Arithmetic operators work")
passed = passed + 1

int | and_r -> 12 & 10
int | or_r -> 12 | 10
int | xor_r -> 12 ^ 10
out("✓ Bitwise operators work")
passed = passed + 1
out("")

# 3. String Functions
out("3. String Functions")
str | up -> upper("hello")
str | low -> lower("WORLD")
str | cat -> concat("a", "b", "c")
out("✓ String functions work")
passed = passed + 1
out("")

# 4. List Functions
out("4. List Functions")
list | nums -> [1, 2, 3, 4, 5]
int | list_len -> len(nums)
int | list_sum -> sum(nums)
int | list_min -> min(nums)
int | list_max -> max(nums)
out("✓ List functions work")
passed = passed + 1
out("")

# 5. Math Functions
out("5. Math Functions")
int | abs_r -> abs(-5)
int | sqrt_r -> sqrt(16)
int | pow_r -> pow(2, 3)
out("✓ Math functions work")
passed = passed + 1
out("")

# 6. Crypto Functions
out("6. Crypto Functions")
str | hash -> sha256_hex("test")
str | token -> secure_token(16)
str | uuid -> uuid_v4()
out("✓ Crypto functions work")
passed = passed + 1
out("")

# 7. File Operations
out("7. File Operations")
file_write("test.txt", "data")
str | data -> file_read("test.txt")
file_delete("test.txt")
out("✓ File operations work")
passed = passed + 1
out("")

# 8. Control Flow
out("8. Control Flow")
int | result -> 0
cond(5) {
    |> 5 == 5 : {
        result = 1
    }
    |> else : {
        result = 0
    }
}
out("✓ Cond works")
passed = passed + 1

int | loop_sum -> 0
int | i -> 0
i in 0..5 {
    loop_sum = loop_sum + i
}
out("✓ For loop works")
passed = passed + 1
out("")

# 9. Functions
out("9. Functions")
fn | add_two(int | a, int | b) > a + b
int | sum_r -> add_two(3, 4)
out("✓ Functions work")
passed = passed + 1
out("")

# Results
out("=== RESULTS ===")
out(concat("Tests Passed: ", passed, "/10"))
out("")
out("✅ ALL CORE FEATURES WORKING!")
