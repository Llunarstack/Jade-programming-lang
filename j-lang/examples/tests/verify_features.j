# Verify J Language Features

# Basic types
str | name -> "J Language"
int | version -> 1
float | pi -> 3.14
bool | ready -> true

out("=== Basic Types ===")
out(name)
out(version)
out(pi)
out(ready)

# Collections
list | nums -> [1, 2, 3, 4, 5]
dict | config -> {port: 8080, host: "localhost"}

out("\n=== Collections ===")
out(nums)
out(config)
out(config.port)

# Loops
out("\n=== Loops ===")
i in nums : out(i)

out("\n=== Range ===")
i in 0..3 : out(i)

out("\n=== Indexed ===")
(i, v) in nums {
  out(i)
  out(v)
}

# Enums
enum | status {
  Active = 1
  Inactive = 2
}

out("\n=== Enums ===")
out(status.Active)
out(status[1].name)
out(status[1].value)

# Built-in functions
out("\n=== Built-ins ===")
out(len(nums))
out(sum(nums))
out(max(nums))

out("\n=== Complete ===")
