out("Grid")
grid | g -> [[1, 2], [3, 4]]
out(g.rows)
out(g.cols)
list | n -> g.neighbors(0, 0)
out(n)

out("Counter")
counter | c -> ["a", "a", "b"]
out(c.most_common)
out(c.total)

out("Defer")
{
  defer out("defer")
  out("block")
}
out("after")

out("Converge")
converge { 42 }
out("done")
