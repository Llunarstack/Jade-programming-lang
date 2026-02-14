# Test class instantiation with .new()

class | Person {
    str | name -> ""
    int | age -> 0
}

# Create instance using .new()
person -> Person.new()
person.name = "Alice"
person.age = 25

out("Person created: " + person.name)
out("Age: " + str(person.age))
