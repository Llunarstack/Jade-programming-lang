// J Language Feature Showcase
// Demonstrates all major features from j.txt

out("=== J Language Feature Showcase ===")
out("")

// 1. Type System
out("1. Type System")
int | count -> 42
float | pi -> 3.14159
str | name -> "J Language"
bool | active -> true
char | initial -> 'J'
float | infinity -> inf
float | neg_infinity -> -inf

out(count)
out(pi)
out(name)
out(active)
out(initial)
out(infinity)
out("")

// 2. Type Conversion
out("2. Type Conversion")
int | num -> 100
str*num
out(num)
out(varType(num))
out("")

// 3. Collections
out("3. Collections")
list | numbers -> [1, 2, 3, 4, 5]
tuple | coords -> (10, 20, 30)
dict | config -> { theme: "dark", size: 14 }

out(numbers)
out(coords)
out(config)
out("")

// 4. Slicing
out("4. Slicing")
list | slice1 -> numbers[0..3]
out(slice1)

list | slice2 -> numbers[..by 2]
out(slice2)

list | reversed -> numbers[4..0 by -1]
out(reversed)
out("")

// 5. Enums
out("5. Enums")
enum | Status {
  Active = 1
  Inactive = 2
  Pending = 3
}

int | current_status -> 1
out(Status[current_status].label)
out("")

// 6. For Loops
out("6. For Loops")

out("Basic:")
i in [1, 2, 3] : out(i)

out("Range:")
i in 0..3 : out(i)

out("Step:")
i in 0..10 by 3 : out(i)

out("Reverse:")
i in [1, 2, 3] rev : out(i)
out("")

// 7. Counter
out("7. Counter")
counter | freq -> ["a", "a", "b", "a", "c", "c"]
out(freq.most_common)
out(freq.total)
out("")

// 8. Counter Arithmetic
out("8. Counter Arithmetic")
counter | c1 -> ["a", "b"]
counter | c2 -> ["a", "c"]
counter | c3 -> c1 + c2
out(c3.most_common)
out("")

// 9. Grid
out("9. Grid")
grid | g -> [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
out(g.rows)
out(g.cols)
out(g.neighbors(1, 1))
out(g.neighbors8(1, 1))
out("")

// 10. Grid Enhanced
out("10. Grid Enhanced")
grid | search -> [[1, 2, 1], [3, 1, 4]]
out(search.find_all(1))
out(g.row(0))
out(g.col(1))
out("")

// 11. Classes
out("11. Classes")
class | Rectangle {
  int | width -> 0
  int | height -> 0
  
  fn | init (int | w, int | h) > {
    this.width = w
    this.height = h
  }
  
  fn | area () > {
    int | result -> this.width * this.height
    out(result)
  }
}

Rectangle | rect -> Rectangle.new(5, 10)
out(rect)
rect.area()
out("")

// 12. Defer
out("12. Defer")
{
  defer out("Cleanup 3")
  defer out("Cleanup 2")
  defer out("Cleanup 1")
  out("Main work")
}
out("")

// 13. Converge
out("13. Converge")
int | value -> 5
converge {
  value = value - 1
  if value <= 0 : break
  value
}
out(value)
out("")

// 14. Window
out("14. Window")
list | data -> [1, 2, 3, 4, 5]
window in windowed(data, 3) : out(window)
out("")

// 15. Dictionaries
out("15. Dictionaries")
dict | person -> {
  name: "Alice"
  age: 30
  address: {
    city: "NYC"
    zip: "10001"
  }
}

out(person.name)
out(person.address.city)
out(keys(person))
out("")

// 16. Functions
out("16. Functions")
fn | add (int | a, int | b) > {
  int | result -> a + b
  result
}

int | sum -> add(5, 3)
out(sum)
out("")

// 17. Lambda
out("17. Lambda")
fn | double -> fn x > x * 2
out(double(21))
out("")

// 18. Pattern Matching
out("18. Pattern Matching")
int | x -> 2
match x {
  1 => out("one")
  2 => out("two")
  _ => out("other")
}
out("")

// 19. Try/Catch
out("19. Try/Catch")
try {
  out("Trying something")
} catch e {
  out("Caught error")
}
out("")

// 20. Printing Features
out("20. Printing Features")
out("Simple text")
out("Colored text", {color: "green"})
out("Bold text", {style: "bold"})
out("")

// 21. Table
out("21. Table")
list | table -> [
  ["Name", "Age"],
  ["Alice", 30],
  ["Bob", 25]
]
out(table)
out("")

// 22. Collection Methods
out("22. Collection Methods")
list | nums -> [1, 2, 3, 4, 5]
out(sum(nums))
out(max(nums))
out(min(nums))
out("")

// 23. Broadcast
out("23. Broadcast")
list | broadcast_result -> add.([1, 2, 3], 10)
out(broadcast_result)
out("")

// 24. Pipeline
out("24. Pipeline")
int | pipeline_result -> 5 |> double |> double
out(pipeline_result)
out("")

// 25. Scan Operations
out("25. Scan Operations")
list | scan_data -> [1, 3, 2, 5, 4]
out(scan_data.scan_max)
out(scan_data.scan_sum)
out("")

out("=== Feature Showcase Complete ===")
out("All major J language features demonstrated!")
