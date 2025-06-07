#!/bin/bash
# Example showcase script for lsdir

echo "=== lsdir Tool Showcase ==="
echo "This directory contains various file types to demonstrate lsdir capabilities"
echo ""

# Create different file types with varying sizes
echo "Creating sample files..."

# Small text files
echo "Hello World" > small_text.txt
echo "This is a test file" > test_file.txt
echo "Another test document" > another_test.txt

# Medium text files
cat << 'EOF' > medium_doc.txt
This is a medium-sized document.
It contains multiple lines of text.
Used for demonstrating file analysis.
Perfect for testing lsdir functionality.
Shows how grouping and filtering work.
EOF

# Large text file
for i in {1..100}; do
    echo "Line $i: This is a large file with many lines for testing purposes." >> large_file.txt
done

# Configuration files
echo "# Configuration file" > config.conf
echo "setting1=value1" >> config.conf
echo "setting2=value2" >> config.conf

echo "# Another config" > app.config
echo "debug=true" >> app.config

# Log files
echo "2024-01-01 10:00:00 INFO Application started" > app.log
echo "2024-01-01 10:01:00 DEBUG Processing request" >> app.log
echo "2024-01-01 10:02:00 ERROR Connection failed" >> app.log

echo "2024-01-01 11:00:00 INFO System check" > system.log
echo "2024-01-01 11:01:00 WARN Memory usage high" >> system.log

# Programming files
cat << 'EOF' > main.py
#!/usr/bin/env python3
"""
Sample Python script for lsdir testing
"""

def main():
    print("Hello from Python!")
    
if __name__ == "__main__":
    main()
EOF

cat << 'EOF' > script.js
// Sample JavaScript file
function greet(name) {
    console.log(`Hello, ${name}!`);
}

greet('World');
EOF

cat << 'EOF' > style.css
/* Sample CSS file */
body {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 20px;
}

.container {
    max-width: 800px;
    margin: 0 auto;
}
EOF

# Data files
echo "name,age,city" > data.csv
echo "John,30,New York" >> data.csv
echo "Jane,25,Los Angeles" >> data.csv
echo "Bob,35,Chicago" >> data.csv

cat << 'EOF' > data.json
{
  "users": [
    {"name": "Alice", "age": 28, "role": "developer"},
    {"name": "Bob", "age": 32, "role": "designer"},
    {"name": "Charlie", "age": 24, "role": "intern"}
  ]
}
EOF

# Binary-like files (simulated)
dd if=/dev/zero of=binary_file.bin bs=1024 count=5 2>/dev/null
dd if=/dev/zero of=large_binary.bin bs=1024 count=50 2>/dev/null

# Backup files
cp small_text.txt backup_file.bak
cp config.conf settings.backup

# Temporary files
echo "temporary content" > temp_file.tmp
echo "another temp" > temp_data.tmp

echo "Sample files created successfully!"
echo ""
echo "Now you can test lsdir with various commands:"
echo "  lsdir examples/showcase"
echo "  lsdir examples/showcase --group-by=extension"
echo "  lsdir examples/showcase --where='size,gt,1000'"
echo "  lsdir examples/showcase --group-by=extension --function=count"
