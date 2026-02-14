# Test static access
class | Math {
  static int | PI -> 3
  
  static fn | add(int | a, int | b) > {
    return a + b
  }
}

out("Accessing static field")
pi -> Math.PI
out(pi)
