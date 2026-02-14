# Test class instantiation
class | Person {
  str | name -> "Unknown"
  int | age -> 0
  
  fn | greet() > {
    out("Hello from Person")
  }
}

out("Class declared successfully")

# Test instantiation
person -> Person.new()
out("Instance created")
out(person)
