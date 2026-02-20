# Create JSON and CSV Files Demo
# This creates files and KEEPS them so you can inspect

out("Creating JSON and CSV files...")
out("")

# ===== CREATE JSON FILES =====
out("1. Creating JSON files")

# Simple JSON array
str | json_numbers -> "[1, 2, 3, 4, 5]"
file_write("numbers.json", json_numbers)
out("✓ Created numbers.json")

# JSON-like data (simple format)
str | json_user -> "name:Alice,age:30,city:NYC"
file_write("user.json", json_user)
out("✓ Created user.json")

# JSON array of items
str | json_items -> "[item1,item2,item3,item4]"
file_write("items.json", json_items)
out("✓ Created items.json")

out("")

# ===== CREATE CSV FILES =====
out("2. Creating CSV files")

# Simple CSV with header
str | csv_users -> "id,name,age,city"
file_write("users.csv", csv_users)
file_append("users.csv", "\n1,Alice,30,NYC")
file_append("users.csv", "\n2,Bob,25,LA")
file_append("users.csv", "\n3,Charlie,35,Chicago")
out("✓ Created users.csv with 3 rows")

# Products CSV
str | csv_products -> "id,product,price,stock"
file_write("products.csv", csv_products)
file_append("products.csv", "\n1,Laptop,999,10")
file_append("products.csv", "\n2,Mouse,25,50")
file_append("products.csv", "\n3,Keyboard,75,30")
out("✓ Created products.csv with 3 rows")

# Sales CSV
str | csv_sales -> "date,product,quantity,total"
file_write("sales.csv", csv_sales)
file_append("sales.csv", "\n2024-01-01,Laptop,2,1998")
file_append("sales.csv", "\n2024-01-02,Mouse,5,125")
file_append("sales.csv", "\n2024-01-03,Keyboard,3,225")
out("✓ Created sales.csv with 3 rows")

out("")

# ===== CREATE CONFIG FILES =====
out("3. Creating config files")

str | config -> "app_name=MyApp\nversion=1.0.0\nport=8080\ndebug=true"
file_write("config.txt", config)
out("✓ Created config.txt")

str | env_config -> "DATABASE_URL=localhost\nAPI_KEY=abc123\nENV=production"
file_write("env.txt", env_config)
out("✓ Created env.txt")

out("")

# ===== CREATE LOG FILE =====
out("4. Creating log file")

int | ts -> timestamp()
str | log1 -> concat(ts, " [INFO] Application started")
file_write("app.log", log1)

str | log2 -> concat("\n", ts, " [INFO] User logged in")
file_append("app.log", log2)

str | log3 -> concat("\n", ts, " [WARN] High memory usage")
file_append("app.log", log3)

str | log4 -> concat("\n", ts, " [INFO] Request processed")
file_append("app.log", log4)
out("✓ Created app.log with 4 entries")

out("")

# ===== CREATE DATA DIRECTORY =====
out("5. Creating data directory")

dir_create("data")
out("✓ Created data/ directory")

file_write("data/notes.txt", "This is a note file")
file_write("data/todo.txt", "1. Task one\n2. Task two\n3. Task three")
file_write("data/readme.txt", "Data directory for application files")
out("✓ Created 3 files in data/")

out("")

# ===== SUMMARY =====
out("╔════════════════════════════════════════╗")
out("║   FILES CREATED SUCCESSFULLY!          ║")
out("╚════════════════════════════════════════╝")
out("")
out("JSON files:")
out("  • numbers.json")
out("  • user.json")
out("  • items.json")
out("")
out("CSV files:")
out("  • users.csv")
out("  • products.csv")
out("  • sales.csv")
out("")
out("Config files:")
out("  • config.txt")
out("  • env.txt")
out("")
out("Log file:")
out("  • app.log")
out("")
out("Directory:")
out("  • data/ (with 3 files)")
out("")
out("Check the j-lang directory to see all files!")
