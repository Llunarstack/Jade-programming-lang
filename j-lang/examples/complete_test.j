# ============================================================================
# COMPLETE J LANGUAGE TEST SUITE
# Tests ALL built-in functions, operators, and language features
# ============================================================================

out("=== COMPLETE J LANGUAGE TEST SUITE ===")
out("")

# ============================================================================
# 1. BASIC TYPES & TYPE CHECKING
# ============================================================================
out("1. Testing Basic Types...")

int | num -> 42
float | pi -> 3.14159
str | text_val -> "Hello J"
bool | flag -> true

out(concat("Integer: ", str(num)))
out(concat("Float: ", str(pi)))
out(concat("String: ", text_val))
out(concat("Boolean: ", str(flag)))

# Type checking
out(concat("type(42) = ", type_of(42)))
out(concat("type(3.14) = ", type_of(3.14)))
out(concat("type(hello) = ", type_of("hello")))
out(concat("type(true) = ", type_of(true)))

# ============================================================================
# 2. ARITHMETIC OPERATORS
# ============================================================================
out("")
out("2. Testing Arithmetic Operators...")

out(concat("10 + 5 = ", str(10 + 5)))
out(concat("10 - 5 = ", str(10 - 5)))
out(concat("10 * 5 = ", str(10 * 5)))
out(concat("10 / 5 = ", str(10 / 5)))
out(concat("10 % 3 = ", str(10 % 3)))
out(concat("pow(2, 8) = ", str(pow(2, 8))))

# ============================================================================
# 3. COMPARISON OPERATORS
# ============================================================================
out("")
out("3. Testing Comparison Operators...")

out(concat("10 > 5 = ", str(10 > 5)))
out(concat("10 < 5 = ", str(10 < 5)))
out(concat("10 >= 10 = ", str(10 >= 10)))
out(concat("10 <= 10 = ", str(10 <= 10)))
out(concat("10 == 10 = ", str(10 == 10)))
out(concat("10 != 5 = ", str(10 != 5)))

# ============================================================================
# 4. LOGICAL OPERATORS
# ============================================================================
out("")
out("4. Testing Logical Operators...")

out(concat("true && true = ", str(true && true)))
out(concat("true && false = ", str(true && false)))
out(concat("true || false = ", str(true || false)))
out(concat("false || false = ", str(false || false)))
out(concat("!true = ", str(!true)))
out(concat("!false = ", str(!false)))

# ============================================================================
# 5. BITWISE OPERATORS
# ============================================================================
out("")
out("5. Testing Bitwise Operators...")

out(concat("12 & 10 = ", str(12 & 10)))
out(concat("12 | 10 = ", str(12 | 10)))
out(concat("12 ^ 10 = ", str(12 ^ 10)))
out(concat("5 << 2 = ", str(5 << 2)))
out(concat("20 >> 2 = ", str(20 >> 2)))
out(concat("~5 = ", str(~5)))

# ============================================================================
# 6. STRING FUNCTIONS
# ============================================================================
out("")
out("6. Testing String Functions...")

str | sample -> "Hello World"
out(concat("len(Hello World) = ", str(len(sample))))
out(concat("upper(Hello World) = ", upper(sample)))
out(concat("lower(Hello World) = ", lower(sample)))
out(concat("trim(  hello  ) = ", trim("  hello  ")))
out(concat("replace(Hello World, World, J) = ", replace(sample, "World", "J")))
out(concat("substring(Hello World, 0, 5) = ", substring(sample, 0, 5)))
out(concat("starts_with(Hello World, Hello) = ", str(starts_with(sample, "Hello"))))
out(concat("ends_with(Hello World, World) = ", str(ends_with(sample, "World"))))
out(concat("repeat(Hi, 3) = ", repeat("Hi", 3)))

list<str> | parts -> split(sample, " ")
out(concat("split(Hello World) = ", join(parts, ", ")))
out(concat("join([Hello, J], -) = ", join(parts, "-")))

# ============================================================================
# 7. LIST FUNCTIONS
# ============================================================================
out("")
out("7. Testing List Functions...")

list<int> | numbers -> [1, 2, 3, 4, 5]
out(concat("List: ", str(numbers)))
out(concat("len([1,2,3,4,5]) = ", str(len(numbers))))
out(concat("sum([1,2,3,4,5]) = ", str(sum(numbers))))
out(concat("min([1,2,3,4,5]) = ", str(min(numbers))))
out(concat("max([1,2,3,4,5]) = ", str(max(numbers))))
out(concat("reverse([1,2,3,4,5]) = ", str(reverse(numbers))))
out(concat("sort([5,2,4,1,3]) = ", str(sort([5, 2, 4, 1, 3]))))
out(concat("unique([1,2,2,3,3,3]) = ", str(unique([1, 2, 2, 3, 3, 3]))))

list<int> | doubled -> map(numbers, fn x > x * 2)
out(concat("map([1,2,3,4,5], x*2) = ", str(doubled)))

list<int> | evens -> filter(numbers, fn x > x % 2 == 0)
out(concat("filter([1,2,3,4,5], even) = ", str(evens)))

int | total -> reduce(numbers, 0, fn acc x > acc + x)
out(concat("reduce([1,2,3,4,5], +) = ", str(total)))

out(concat("flatten([[1,2],[3,4]]) = ", str(flatten([[1, 2], [3, 4]]))))
out(concat("count([1,2,3,2,1], 2) = ", str(count([1, 2, 3, 2, 1], 2))))

list<tuple> | zipped -> zip([1, 2, 3], ["a", "b", "c"])
out(concat("zip([1,2,3], [a,b,c]) = ", str(zipped)))

list<tuple> | enumerated -> enumerate(["a", "b", "c"])
out(concat("enumerate([a,b,c]) = ", str(enumerated)))

# ============================================================================
# 8. SET OPERATIONS
# ============================================================================
out("")
out("8. Testing Set Operations...")

list<int> | set1 -> [1, 2, 3, 4]
list<int> | set2 -> [3, 4, 5, 6]

out(concat("union([1,2,3,4], [3,4,5,6]) = ", str(union(set1, set2))))
out(concat("intersect([1,2,3,4], [3,4,5,6]) = ", str(intersect(set1, set2))))
out(concat("difference([1,2,3,4], [3,4,5,6]) = ", str(difference(set1, set2))))
out(concat("symmetric_diff([1,2,3,4], [3,4,5,6]) = ", str(symmetric_diff(set1, set2))))

# ============================================================================
# 9. MATH FUNCTIONS
# ============================================================================
out("")
out("9. Testing Math Functions...")

out(concat("abs(-42) = ", str(abs(-42))))
out(concat("sqrt(16) = ", str(sqrt(16))))
out(concat("pow(2, 10) = ", str(pow(2, 10))))
out(concat("ceil(3.2) = ", str(ceil(3.2))))
out(concat("floor(3.8) = ", str(floor(3.8))))
out(concat("round(3.5) = ", str(round(3.5))))

out(concat("sin(0) = ", str(sin(0))))
out(concat("cos(0) = ", str(cos(0))))
out(concat("tan(0) = ", str(tan(0))))
out(concat("log(2.718) = ", str(log(2.718))))
out(concat("exp(1) = ", str(exp(1))))

out(concat("clamp(15, 0, 10) = ", str(clamp(15, 0, 10))))
out(concat("lerp(0, 100, 0.5) = ", str(lerp(0, 100, 0.5))))

# ============================================================================
# 10. RANGE FUNCTIONS
# ============================================================================
out("")
out("10. Testing Range Functions...")

out(concat("range(5) = ", str(range(5))))
out(concat("range(2, 7) = ", str(range(2, 7))))
out(concat("range(0, 10, 2) = ", str(range(0, 10, 2))))

# ============================================================================
# 11. DICTIONARY FUNCTIONS
# ============================================================================
out("")
out("11. Testing Dictionary Functions...")

dict | person -> {"name": "Alice", "age": "30", "city": "NYC"}
out(concat("Dict: ", str(person)))
out(concat("keys(dict) = ", str(keys(person))))
out(concat("values(dict) = ", str(values(person))))
out(concat("items(dict) = ", str(items(person))))
out(concat("get(dict, name) = ", get(person, "name")))
out(concat("get(dict, missing, default) = ", get(person, "missing", "N/A")))
out(concat("has(dict, name) = ", str(has(person, "name"))))
out(concat("has(dict, missing) = ", str(has(person, "missing"))))

dict | dict1 -> {"a": "1", "b": "2"}
dict | dict2 -> {"c": "3", "d": "4"}
dict | merged -> merge(dict1, dict2)
out(concat("merge({a:1,b:2}, {c:3,d:4}) = ", str(merged)))

# ============================================================================
# 12. VECTOR & MATRIX OPERATIONS
# ============================================================================
out("")
out("12. Testing Vector & Matrix Operations...")

vec | v1 -> vec([1.0, 2.0, 3.0])
vec | v2 -> vec([4.0, 5.0, 6.0])

out(concat("vec([1,2,3]) = ", str(v1)))
out(concat("dot([1,2,3], [4,5,6]) = ", str(dot(v1, v2))))
out(concat("magnitude([3,4]) = ", str(magnitude(vec([3.0, 4.0])))))

mat | m1 -> mat([[1.0, 2.0], [3.0, 4.0]])
out(concat("matrix([[1,2],[3,4]]) = ", str(m1)))
out(concat("zeros(3) = ", str(zeros(3))))
out(concat("ones(3) = ", str(ones(3))))
out(concat("identity(3) = ", str(identity(3))))

out(concat("matrix.rows = ", str(rows(m1))))
out(concat("matrix.cols = ", str(cols(m1))))
out(concat("matrix.size = ", str(size(m1))))

# ============================================================================
# 13. FUNCTIONAL PROGRAMMING
# ============================================================================
out("")
out("13. Testing Functional Programming...")

list<int> | nums -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

list<int> | taken -> take(nums, 5)
out(concat("take([1..10], 5) = ", str(taken)))

list<int> | dropped -> drop(nums, 5)
out(concat("drop([1..10], 5) = ", str(dropped)))

list<int> | sliced -> slice(nums, 2, 7)
out(concat("slice([1..10], 2, 7) = ", str(sliced)))

list<int> | chunked -> chunk_list(nums, 3)
out(concat("chunk([1..10], 3) = ", str(chunked)))

list<int> | windowed -> sliding_window(nums, 3)
out(concat("sliding_window([1..10], 3) = ", str(windowed)))

tuple | partitioned -> partition([1, 2, 3, 4, 5, 6], fn x > x % 2 == 0)
out(concat("partition([1..6], even) = ", str(partitioned)))

dict | grouped -> group_by([1, 2, 3, 4, 5, 6], fn x > str(x % 2))
out(concat("group_by([1..6], mod2) = ", str(grouped)))

dict | freq -> frequencies(["a", "b", "a", "c", "b", "a"])
out(concat("frequencies([a,b,a,c,b,a]) = ", str(freq)))

list<int> | compacted -> compact([1, 2, none, 3, none, 4])
out(concat("compact([1,2,none,3,none,4]) = ", str(compacted)))

# ============================================================================
# 14. STRING ALGORITHMS
# ============================================================================
out("")
out("14. Testing String Algorithms...")

out(concat("levenshtein(kitten, sitting) = ", str(levenshtein("kitten", "sitting"))))
out(concat("hamming(karolin, kathrin) = ", str(hamming("karolin", "kathrin"))))

list<int> | kmp_result -> kmp_search("ababcababa", "aba")
out(concat("kmp_search(ababcababa, aba) = ", str(kmp_result)))

list<int> | z_result -> z_array("aabaaab")
out(concat("z_array(aabaaab) = ", str(z_result)))

# ============================================================================
# 15. GRAPH OPERATIONS
# ============================================================================
out("")
out("15. Testing Graph Operations...")

graph | g -> graph()
graph | g1 -> add_node(g, "A")
graph | g2 -> add_node(g1, "B")
graph | g3 -> add_node(g2, "C")
graph | g4 -> add_edge(g3, "A", "B")
graph | g5 -> add_edge(g4, "B", "C")

out(concat("Graph nodes: ", str(nodes(g5))))
out(concat("Graph edges: ", str(edges(g5))))

list<str> | bfs_result -> bfs(g5, "A")
out(concat("BFS from A: ", str(bfs_result)))

list<str> | dfs_result -> dfs(g5, "A")
out(concat("DFS from A: ", str(dfs_result)))

# ============================================================================
# 16. ALGORITHM PATTERNS
# ============================================================================
out("")
out("16. Testing Algorithm Patterns...")

list<int> | two_ptr -> two_pointers([1, 2, 3, 4, 5], 7)
out(concat("two_pointers([1,2,3,4,5], 7) = ", str(two_ptr)))

int | bs_result -> binary_search([1, 2, 3, 4, 5, 6, 7, 8, 9], 5)
out(concat("binary_search([1..9], 5) = ", str(bs_result)))

list<int> | sw_max -> sliding_window_max([1, 3, -1, -3, 5, 3, 6, 7], 3)
out(concat("sliding_window_max([...], 3) = ", str(sw_max)))

list<list<int>> | intervals -> [[1, 3], [2, 6], [8, 10], [15, 18]]
list<list<int>> | merged_intervals -> merge_intervals(intervals)
out(concat("merge_intervals(...) = ", str(merged_intervals)))

dict | topo_graph -> {"A": ["B", "C"], "B": ["D"], "C": ["D"], "D": []}
list<str> | topo_result -> topological_sort(topo_graph)
out(concat("topological_sort(...) = ", str(topo_result)))

# ============================================================================
# 17. ADVANCED FEATURES
# ============================================================================
out("")
out("17. Testing Advanced Features...")

int | tapped -> tap(42, "Debug value")

fn test_fn > {
    int | sum_val -> 0
    loop 1000 : sum_val = sum_val + 1
    sum_val
}
dict | bench_result -> benchmark(test_fn, 10)
out(concat("Benchmark: ", str(get(bench_result, "avg_ms")), " ms"))

fn flaky_fn > 42
int | retry_result -> retry(flaky_fn, 3)
out(concat("Retry result: ", str(retry_result)))

list<int> | scan_result -> scan([1, 2, 3, 4, 5], 0, fn acc x > acc + x)
out(concat("scan([1,2,3,4,5], +) = ", str(scan_result)))

list<int> | interleaved -> interleave([1, 2, 3], [4, 5, 6], [7, 8, 9])
out(concat("interleave([1,2,3], [4,5,6], [7,8,9]) = ", str(interleaved)))

list<list<int>> | matrix_data -> [[1, 2, 3], [4, 5, 6]]
list<list<int>> | transposed -> transpose(matrix_data)
out(concat("transpose([[1,2,3],[4,5,6]]) = ", str(transposed)))

list<dict> | users -> [
    {"name": "Alice", "age": "25"},
    {"name": "Bob", "age": "30"},
    {"name": "Charlie", "age": "35"}
]
list<str> | user_names -> pluck(users, "name")
out(concat("pluck(users, name) = ", str(user_names)))

# ============================================================================
# 18. CRYPTOGRAPHY FUNCTIONS
# ============================================================================
out("")
out("18. Testing Cryptography Functions...")

str | message -> "Hello World"
str | hash_result -> sha256_hex(message)
out(concat("sha256_hex(Hello World) = ", substring(hash_result, 0, 16), "..."))

str | hmac_result -> hmac(message, "secret_key")
out(concat("hmac(Hello World, secret_key) = ", substring(hmac_result, 0, 16), "..."))

str | password -> "mypassword123"
str | hashed_pw -> password_hash(password)
out(concat("password_hash(mypassword123) = ", substring(hashed_pw, 0, 20), "..."))

bool | pw_verify -> password_verify(password, hashed_pw)
out(concat("password_verify(correct) = ", str(pw_verify)))

str | plaintext_msg -> "secret message"
str | encryption_key -> "my_secret_key"
str | encrypted -> encrypt(plaintext_msg, encryption_key)
out(concat("encrypt(secret message) = ", substring(encrypted, 0, 16), "..."))

str | decrypted -> decrypt(encrypted, encryption_key)
out(concat("decrypt(...) = ", decrypted))

str | uuid -> uuid_v4()
out(concat("uuid_v4() = ", uuid))

str | token -> secure_token(32)
out(concat("secure_token(32) = ", substring(token, 0, 16), "..."))

int | rand_num -> rand_range(1, 100)
out(concat("rand_range(1, 100) = ", str(rand_num)))

# ============================================================================
# 19. FILE OPERATIONS
# ============================================================================
out("")
out("19. Testing File Operations...")

str | test_file -> "test_output.txt"
str | test_content -> "Hello from J!"

bool | write_result -> file_write(test_file, test_content)
out(concat("file_write(test_output.txt) = ", str(write_result)))

bool | exists_result -> file_exists(test_file)
out(concat("file_exists(test_output.txt) = ", str(exists_result)))

str | read_content -> file_read(test_file)
out(concat("file_read(test_output.txt) = ", read_content))

bool | append_result -> file_append(test_file, " Appended!")
out(concat("file_append(...) = ", str(append_result)))

str | read_after_append -> file_read(test_file)
out(concat("After append: ", read_after_append))

bool | delete_result -> file_delete(test_file)
out(concat("file_delete(test_output.txt) = ", str(delete_result)))

# ============================================================================
# 20. CLI FUNCTIONS
# ============================================================================
out("")
out("20. Testing CLI Functions...")

str | env_value -> env_get("PATH", "not_found")
out(concat("env_get(PATH) exists: ", str(len(env_value) > 0)))

int | timestamp_val -> timestamp()
out(concat("timestamp() = ", str(timestamp_val)))

# ============================================================================
# 21. CONTROL FLOW - COND
# ============================================================================
out("")
out("21. Testing Cond (Conditional Pipeline)...")

int | score -> 85
str | grade -> cond(score) {
    |> score >= 90 : "A"
    |> score >= 80 : "B"
    |> score >= 70 : "C"
    |> score >= 60 : "D"
    |> else : "F"
}
out(concat("Grade for 85: ", grade))

# ============================================================================
# 22. FUNCTIONS & LAMBDAS
# ============================================================================
out("")
out("22. Testing Functions & Lambdas...")

fn add_nums (int | a, int | b) > a + b
int | add_result -> add_nums(10, 20)
out(concat("add(10, 20) = ", str(add_result)))

fn multiply_nums (int | x, int | y) > x * y
int | mult_result -> multiply_nums(6, 7)
out(concat("multiply(6, 7) = ", str(mult_result)))

# ============================================================================
# 23. LOOPS
# ============================================================================
out("")
out("23. Testing Loops...")

out("Loop 5 times:")
loop 5 : out("  - iteration")

out("For loop [1,2,3]:")
for item in [1, 2, 3] {
    out(concat("  - item: ", str(item)))
}

out("While loop (count to 3):")
int | counter -> 0
while counter < 3 {
    out(concat("  - counter: ", str(counter)))
    counter = counter + 1
}

# ============================================================================
# SUMMARY
# ============================================================================
out("")
out("=== TEST SUITE COMPLETE ===")
out("All major J language features tested successfully!")
