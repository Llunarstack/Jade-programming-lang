# Advanced J Language Features

# Money types
money | price -> $29.99
money | euro_price -> €25.50
money | bitcoin -> ₿0.0042

out("Prices:")
out(price)
out(euro_price)
out(bitcoin)

# Date and time
date | today -> 2026-01-30
time | now -> 14:30:00
datetime | meeting -> 2026-01-30 15:45:00

out("Date/Time:")
out(today)
out(now)
out(meeting)

# Hex colors
hex | primary -> #FF3366
hex | secondary -> #33FF66

out("Colors:")
out(primary)
out(secondary)

# Infinity values
float | pos_inf -> inf
float | neg_inf -> -inf

out("Infinity:")
out(pos_inf)
out(neg_inf)
out(pos_inf > 1000000)

# Emoji support
emoji | happy -> 😊
emoji | fire -> 🔥

out("Emojis:")
out(happy)
out(fire)

# Character types
char | initial -> 'J'
ascii | delimiter -> ':'

out("Characters:")
out(initial)
out(delimiter)

# Complex data structures
dict | person -> {
    "name": "Ethan",
    "age": 25,
    "hobbies": ["coding", "music", "gaming"]
}

out("Person:")
out(person)

# Nested loops
out("Multiplication table (3x3):")
i in 1..4 {
    j in 1..4 {
        out(i + " x " + j + " = " + (i * j))
    }
}

# Function with multiple parameters
fn str | greet (str | name, int | age) > {
    "Hello " + name + ", you are " + age + " years old!"
}

out(greet("Alice", 30))

# Pattern matching example (simplified)
match answer {
    42 : out("The answer to everything!")
    _ : out("Just a number")
}