# CLI Tools Demo

out("=== CLI Tools Demo ===")
out("")

# 1. Command line arguments
out("1. Command Line Arguments")
list | args -> cli_args()
out("Arguments passed:")
out(args)
out("")

# 2. Interactive prompt
out("2. Interactive Input")
str | name -> cli_prompt("What is your name? ")
out("Hello, " + name + "!")
out("")

# 3. Environment variables
out("3. Environment Variables")
str | home -> env_get("HOME")
out("HOME directory: " + home)

str | user -> env_get("USER")
out("Current user: " + user)
out("")

# 4. File operations
out("4. File Operations")
str | test_file -> "test_output.txt"
str | content -> "Hello from J!\nThis is a test file.\n"

file_write(test_file, content)
out("Wrote to " + test_file)

bool | exists -> file_exists(test_file)
out("File exists: " + str(exists))

str | read_content -> file_read(test_file)
out("File content:")
out(read_content)
out("")

# 5. Directory listing
out("5. Directory Listing")
list | files -> dir_list(".")
out("Files in current directory:")
out(len(files))
out("")

# 6. Timestamp
out("6. Timestamp")
int | ts -> timestamp()
out("Current Unix timestamp: " + str(ts))
out("")

# 7. Sleep
out("7. Sleep Test")
out("Sleeping for 1 second...")
sleep(1000)
out("Done!")
out("")

out("=== CLI Demo Complete ===")
