# Test arrow lambda syntax
list | nums -> [1, 2, 3, 4, 5]

# Test arrow lambda
list | doubled -> nums |> map(x => x * 2)
out(doubled)

# Test with multiple params
list | pairs -> [(1, 2), (3, 4), (5, 6)]
# pairs |> map((a, b) => a + b) |> out

out("Arrow lambda test complete")
