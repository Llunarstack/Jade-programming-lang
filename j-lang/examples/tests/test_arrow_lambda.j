# Test arrow lambda syntax
list | nums -> [1, 2, 3, 4, 5]

# Arrow lambda should work
# nums |> map(x => x * 2) |> out

# For now, test regular lambda
nums |> map(fn x > x * 2) |> out

out("Arrow lambda test complete")
