# Cond Demonstration

out("=== Cond Conditional Pipeline ===")
out("")

# Example 1: Grade System
out("1. Grade System")
int | score1 -> 95
cond(score1) {
    |> score1 >= 90 : out("Grade: A")
    |> score1 >= 80 : out("Grade: B")
    |> score1 >= 70 : out("Grade: C")
    |> score1 >= 60 : out("Grade: D")
    |> else         : out("Grade: F")
}

int | score2 -> 75
cond(score2) {
    |> score2 >= 90 : out("Grade: A")
    |> score2 >= 80 : out("Grade: B")
    |> score2 >= 70 : out("Grade: C")
    |> score2 >= 60 : out("Grade: D")
    |> else         : out("Grade: F")
}

int | score3 -> 45
cond(score3) {
    |> score3 >= 90 : out("Grade: A")
    |> score3 >= 80 : out("Grade: B")
    |> score3 >= 70 : out("Grade: C")
    |> score3 >= 60 : out("Grade: D")
    |> else         : out("Grade: F")
}
out("")

# Example 2: Temperature Control
out("2. Temperature Control")
int | temp1 -> 105
cond(temp1) {
    |> temp1 > 100 : out("Status: Critical overheat!")
    |> temp1 > 80  : out("Status: Hot")
    |> temp1 < 0   : out("Status: Freezing")
    |> else        : out("Status: Normal")
}

int | temp2 -> 85
cond(temp2) {
    |> temp2 > 100 : out("Status: Critical overheat!")
    |> temp2 > 80  : out("Status: Hot")
    |> temp2 < 0   : out("Status: Freezing")
    |> else        : out("Status: Normal")
}

int | temp3 -> 25
cond(temp3) {
    |> temp3 > 100 : out("Status: Critical overheat!")
    |> temp3 > 80  : out("Status: Hot")
    |> temp3 < 0   : out("Status: Freezing")
    |> else        : out("Status: Normal")
}
out("")

# Example 3: Sign Detection
out("3. Sign Detection")
int | x1 -> 0
cond(x1) {
    |> x1 == 0 : out("zero")
    |> x1 > 0  : out("positive")
    |> else    : out("negative")
}

int | x2 -> 42
cond(x2) {
    |> x2 == 0 : out("zero")
    |> x2 > 0  : out("positive")
    |> else    : out("negative")
}

int | x3 -> -15
cond(x3) {
    |> x3 == 0 : out("zero")
    |> x3 > 0  : out("positive")
    |> else    : out("negative")
}
out("")

# Example 4: Multi-line Blocks
out("4. Multi-line Blocks")
int | value -> 150
cond(value) {
    |> value > 100 : {
        out("Value is large")
        out("Processing large value...")
    }
    |> value > 50 : {
        out("Value is medium")
        out("Processing medium value...")
    }
    |> else : {
        out("Value is small")
        out("Processing small value...")
    }
}
out("")

# Example 5: Using with comparisons
out("5. Day of Week")
int | day -> 3
cond(day) {
    |> day == 1 : out("Monday")
    |> day == 2 : out("Tuesday")
    |> day == 3 : out("Wednesday")
    |> day == 4 : out("Thursday")
    |> day == 5 : out("Friday")
    |> day == 6 : out("Saturday")
    |> day == 7 : out("Sunday")
    |> else     : out("Invalid day")
}
out("")

out("=== All Cond Examples Complete ===")
