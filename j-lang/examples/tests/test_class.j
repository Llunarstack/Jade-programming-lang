# Test class declaration
class | Person {
  str | name -> "Unknown"
  int | age -> 0
  
  fn | greet() > {
    out("Hello from Person")
  }
}

out("Class declared successfully")
