# Test class instantiation only
class | Person {
  str | name -> "Unknown"
  int | age -> 0
}

out("About to create instance")
person -> Person.new()
out("Instance created!")
