# NUMPY-STYLE MATRIX OPERATIONS IN J
# Demonstrating clean, powerful matrix manipulation

out("========================================")
out("J LANGUAGE - NumPy-Style Matrix Operations")
out("========================================\n")

# Create a 3x3 matrix
mat | data -> [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]

out("Original 3x3 Matrix:")
out(data)
out("Shape:", data.rows, "x", data.cols, "\n")

# Quick access patterns
out("=== QUICK ACCESS ===")
out("First row:", data.row(0))
out("Last row:", data.row(-1))
out("Middle column:", data.col(1))
out("Diagonal:", data.diagonal())
out()

# Statistical operations
out("=== STATISTICS ===")
out("Row sums:", data.row_sums())
out("Column sums:", data.col_sums())
out("Row means:", data.row_means())
out("Column means:", data.col_means())
out()

# Transformations
out("=== TRANSFORMATIONS ===")
out("Transposed:")
out(data.T)
out("\nFlattened:", data.flat())
out()

# Working with non-square matrices
out("=== NON-SQUARE MATRIX (2x5) ===")
mat | wide -> [[1.0, 2.0, 3.0, 4.0, 5.0], [6.0, 7.0, 8.0, 9.0, 10.0]]
out("Matrix:", wide)
out("Shape:", wide.rows, "x", wide.cols)
out("Column 3:", wide.col(3))
out("Row means:", wide.row_means())
out()

# Combining with other operations
out("=== COMBINING OPERATIONS ===")
mat | m -> [[2.0, 4.0], [6.0, 8.0]]
out("Matrix:", m)
out("Diagonal:", m.diagonal())
list | diag_vals -> m.diagonal()
out("Diagonal values:", diag_vals)
out("Transposed diagonal:", m.T.diagonal())
out()

# Matrix arithmetic (already supported)
out("=== MATRIX ARITHMETIC ===")
mat | a -> [[1.0, 2.0], [3.0, 4.0]]
mat | b -> [[5.0, 6.0], [7.0, 8.0]]
out("Matrix A:", a)
out("Matrix B:", b)
out("A + B:", a + b)
out("A * 2:", a * 2.0)
out()

out("========================================")
out("J makes matrices as easy as Python!")
out("========================================")
