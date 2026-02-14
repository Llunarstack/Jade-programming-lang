# File execution example

str | main_file -> "file_execution.j"
out("This is the main file: " + main_file)

# Execute another J file
j; -> utils.j

# Use functions from the executed file
out("Using function from utils.j:")
out(double(21))