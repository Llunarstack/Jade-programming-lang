# Simple class test
class | Person {
    str | name -> "Unknown"
    int | age -> 0
}

person -> Person.new()
out("Instance created")
out(person)