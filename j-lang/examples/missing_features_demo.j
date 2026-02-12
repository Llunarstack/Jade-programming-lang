// Demo: Missing features (Phase 1 & 2)

// --- Phase 1: Core OOP ---
class | Point {
  int | x -> 0
  int | y -> 0
  fn | init (int | a, int | b) > { out("init called") }
  fn | dist () > { out("dist") }
}

out(Point.new(3, 4))
out("OOP: ClassName.new() works")

// --- Phase 2: Grid with .neighbors() ---
grid | g -> [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
out(g.rows)
out(g.cols)
list | neighbors -> g.neighbors(1, 1)
out(neighbors)

// --- Counter methods ---
counter | c -> ["a", "a", "b", "a", "c"]
list | top -> c.most_common
out(top)
out(c.total)
out(c.elements)

// --- Defer (LIFO on block exit) ---
out("before block")
{
  defer out("defer 2")
  defer out("defer 1")
  out("inside block")
}
out("after block")

// --- Converge loop ---
converge { 42 }
out("converge done")
