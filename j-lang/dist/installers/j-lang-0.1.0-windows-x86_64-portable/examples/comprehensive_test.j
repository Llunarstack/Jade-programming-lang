// Comprehensive test of J language features from j.txt

out("=== Testing Core Features ===")

// Variables
str | name -> "J Language"
int | version -> 1
float | pi -> 3.14159
bool | active -> true
out(name)
out(version)
out(pi)
out(active)

// Type conversion
int | count -> 42
str*count
out("Type conversion:")
out(count)
out(varType(count))

// Collections
list | nums -> [1, 2, 3, 4, 5]
dict | config -> { theme: "dark", size: 14 }
tuple | point -> (10, 20)
out(nums)
out(config)
out(point)

// Slicing with step
list | evens -> nums[0..5 by 2]
out("Evens:")
out(evens)

list | reversed -> nums[4..0 by -1]
out("Reversed:")
out(reversed)

// Infinity
float | inf_val -> inf
float | neg_inf_val -> -inf
out("Infinity:")
out(inf_val)
out(neg_inf_val)

// Enums
enum | Direction {
  North = 1
  South = 2
  East = 3
  West = 4
}

int | heading -> 3
out("Direction:")
out(Direction[heading].label)

// For loops - basic
out("=== For Loops ===")
i in nums : out(i)

// For loops - indexed
(i, v) in nums : out(v)

// For loops - range
i in 0..5 : out(i)

// For loops - step
i in 0..10 by 2 : out(i)

// For loops - reverse
i in nums rev : out(i)

// Nested loops
out("=== Nested Loops ===")
r in [1, 2] : c in [1, 2] : out(c)

// Counter
out("=== Counter ===")
counter | freq -> ["a", "a", "b", "a", "c", "c"]
out("Most common:")
out(freq.most_common)
out("Total:")
out(freq.total)
out("Elements:")
out(freq.elements)

// Counter arithmetic
counter | c1 -> ["a", "b"]
counter | c2 -> ["a", "c"]
counter | c3 -> c1 + c2
out("Counter sum:")
out(c3.most_common)

// Grid
out("=== Grid ===")
grid | g -> [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
out("Grid rows:")
out(g.rows)
out("Grid cols:")
out(g.cols)
out("Neighbors of (1,1):")
out(g.neighbors(1, 1))
out("8-neighbors of (1,1):")
out(g.neighbors8(1, 1))

// Grid find_all
grid | search_grid -> [[1, 2, 1], [3, 1, 4], [1, 5, 1]]
list | positions -> search_grid.find_all(1)
out("Positions of 1:")
out(positions)

// Grid row and col
out("Row 0:")
out(g.row(0))
out("Col 1:")
out(g.col(1))

// Classes
out("=== Classes ===")
class | Point {
  int | x -> 0
  int | y -> 0
  
  fn | init (int | a, int | b) > {
    this.x = a
    this.y = b
  }
  
  fn | distance () > {
    out("Calculating distance")
  }
}

Point | p -> Point.new(3, 4)
out("Point created:")
out(p)
p.distance()

// Defer
out("=== Defer ===")
{
  defer out("Cleanup 2")
  defer out("Cleanup 1")
  out("Main work")
}

// Converge loop
out("=== Converge ===")
int | val -> 10
converge {
  val = val - 1
  if val <= 0 : break
  val
}
out("Converged to:")
out(val)

// Window loop
out("=== Window ===")
list | data -> [1, 2, 3, 4, 5]
window in windowed(data, 3) : out(window)

// Dictionaries - nested access
out("=== Nested Dict Access ===")
dict | user -> {
  name: "Alex"
  age: 29
  address: {
    city: "Portland"
    zip: "04101"
  }
  hobbies: ["coding", "music"]
}

out("Name:")
out(user.name)
out("City:")
out(user.address.city)
out("First hobby:")
out(user.hobbies[0])

// Dict keys
out("Keys:")
out(keys(user))

// Printing features
out("=== Printing Features ===")
out("Simple message")
out("Red text", {color: "red"})
out("Bold text", {style: "bold"})

// Table
out("=== Table ===")
list | table_data -> [
  ["Name", "Age", "City"],
  ["Alice", 25, "NYC"],
  ["Bob", 30, "LA"]
]
out(table_data)

// Progress bar
out("=== Progress ===")
out("Progress bar:", {progress: 50.0, width: 30})

// Gradient
out("=== Gradient ===")
out("Gradient text", {gradient: ["#FF0066", "#00FF99"]})

out("=== All Tests Complete ===")
