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

out("Broadcast (from jnew_features)")
fn int | add (int | a, int | b) > a + b
list | nums -> [1, 2, 3]
list | results -> add.(nums, 10)
out(results)
out("Double broadcast (zip):")
list | b -> [10, 20, 30]
list | results2 -> add.(nums, b)
out(results2)

out("scan_max / scan_sum (algorithm features)")
list | h -> [1, 3, 2, 5, 4]
out(h.scan_max)
out(h.scan_sum)
out(h.scan_right_max)

out("Secure block + constant-time eq ~==")
str | a -> "secret"
str | b -> "secret"
if a ~== b { out("match") }

out("Rollback (stub)")
rollback { out("rollback body") }

out("Retry (stub)")
retry { out("retry body") }

out("Race (first branch)")
race { "A" : 1 }

out("Memo variable (callable)")
memo int | double (int | n) -> n * 2
out(double(5))
out(double(7))

out("Task (runs body)")
task | hello > { out("task ran") }

out("Fuzz loop (stub: 100 iters)")
fuzz int | n : n >= 0

out("Within loop (over list)")
within 1000 | x in [10, 20, 30] { out(x) }

out("@once decorator (caches first call)")
@once
fn int | once_val () > 99
out(once_val())
out(once_val())

out("value.defer(cleanup)")
{
  int | r -> 42.defer(out("cleanup"))
  out(r)
}
out("after block")

out("All jnew_features syntax accepted.")
