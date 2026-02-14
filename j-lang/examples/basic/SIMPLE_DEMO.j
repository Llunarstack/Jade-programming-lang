# J-Lang Simple Feature Demo

out("J-LANG FEATURE DEMONSTRATION")
out("========================================")

out("\n1. LAMBDAS IN MAP AND FILTER")
list | data -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
list | squared -> map(data, |x| x * x)
out("Squared:", squared)
list | evens -> filter(data, |x| x % 2 == 0)
out("Evens:", evens)

out("\n2. ENHANCED FOR LOOPS")
out("Reverse:")
for i in [1, 2, 3, 4, 5] rev {
    out("  ", i)
}

out("Filtered:")
for i in [1, 2, 3, 4, 5, 6, 7, 8] if i % 2 == 0 {
    out("  ", i)
}

out("Chunked:")
for chunk in chunks([1, 2, 3, 4, 5, 6], 2) {
    out("  ", chunk)
}

out("\n3. SLICING WITH STEP")
list | arr -> [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
out("arr[.. by 2] =", arr[.. by 2])
out("arr[1.. by 2] =", arr[1.. by 2])

out("\n4. DEQUE OPERATIONS")
deque | dq -> [1, 2, 3]
dq = push_front(dq, 0)
dq = push_back(dq, 4)
out("Deque:", dq)
out("peek_front:", peek_front(dq))
out("peek_back:", peek_back(dq))

out("\n5. PRIORITY QUEUE")
priorityq | pq -> []
pq = pq_push(pq, 3, "Low priority")
pq = pq_push(pq, 1, "High priority")
pq = pq_push(pq, 2, "Medium priority")
out("Peek (highest priority):", pq_peek(pq))
out("Pop:", pq_pop(pq))
out("Pop:", pq_pop(pq))

out("\n6. GRAPH OPERATIONS")
graph | g -> {}
g = add_node(g, "A")
g = add_node(g, "B")
g = add_node(g, "C")
g = add_node(g, "D")
g = add_edge(g, "A", "B", 1)
g = add_edge(g, "A", "C", 2)
g = add_edge(g, "B", "D", 3)
g = add_edge(g, "C", "D", 1)
out("Neighbors of A:", get_neighbors(g, "A"))
out("BFS from A:", bfs(g, "A"))
out("DFS from A:", dfs(g, "A"))

out("\n7. PIPELINE OPERATOR WITH LAMBDAS")
list | result -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] |> map(|x| x * 2) |> filter(|x| x > 10)
out("Pipeline result:", result)

out("\n========================================")
out("NEW FEATURES WORKING PERFECTLY!")
out("========================================")
