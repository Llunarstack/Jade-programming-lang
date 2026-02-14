# Test class instantiation

class | Person {
    str | name -> "Unknown"
    int | age -> 0
    
    fn | greet () > {
        out("Hello, I am " name)
    }
}

# Create an instance
person1 -> Person.new("Alice", 30)

out("Created person:")
out(person1)
out(varType(person1))
