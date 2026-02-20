# Test that the easter egg fails silently with wrong params

out("Before quack_check")

# Wrong sound
quack_check("meow", 4, ["quack", "quack", "quack", "quack"])

# Wrong number
quack_check("quack", 3, ["quack", "quack", "quack", "quack"])

# Wrong list
quack_check("quack", 4, ["duck", "duck", "duck", "duck"])

# Wrong argument count
quack_check("quack", 4)

out("After quack_check - nothing happened!")
