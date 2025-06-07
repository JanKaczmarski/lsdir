# lsdir Examples and Tutorial

This directory contains comprehensive examples demonstrating all features of the `lsdir` tool. The examples use the sample files in the `showcase/` directory.

## Quick Start

First, make sure you have built the project:

```bash
cargo build --release
```

Then create the sample files:

```bash
cd examples/showcase
../create_samples.sh
```

## Basic Examples

### 1. Simple File Listing

```bash
# List all files in the showcase directory
./target/release/lsdir examples/showcase
```

**Expected Output:**

```
Files (21 total):
----------------------------------------
small_text.txt                 File               12 bytes
test_file.txt                  File               19 bytes
another_test.txt               File               21 bytes
medium_doc.txt                 File              184 bytes
large_file.txt                 File             7000 bytes
...
```

### 2. Counting Files

```bash
# Count total number of files
./target/release/lsdir examples/showcase --function=count
```

**Expected Output:**

```
Count: 21
```

## Filtering Examples

### 3. Size-based Filtering

```bash
# Files larger than 1000 bytes
./target/release/lsdir examples/showcase --where="size,gt,1000"
```

**Expected Output:**

```
Files (3 total):
----------------------------------------
large_file.txt                 File             7000 bytes
large_binary.bin               File            51200 bytes
binary_file.bin                File             5120 bytes
```

### 4. Extension-based Filtering

```bash
# Only text files
./target/release/lsdir examples/showcase --where="extension,eq,txt"
```

**Expected Output:**

```
Files (4 total):
----------------------------------------
small_text.txt                 File               12 bytes
test_file.txt                  File               19 bytes
another_test.txt               File               21 bytes
medium_doc.txt                 File              184 bytes
large_file.txt                 File             7000 bytes
```

### 5. Name Pattern Matching

```bash
# Files containing "test" in name
./target/release/lsdir examples/showcase --where="name,contains,test"
```

**Expected Output:**

```
Files (3 total):
----------------------------------------
test_file.txt                  File               19 bytes
another_test.txt               File               21 bytes
```

### 6. Wildcard Pattern Matching

```bash
# Files starting with "temp"
./target/release/lsdir examples/showcase --where="name,starts_with,temp"
```

**Expected Output:**

```
Files (2 total):
----------------------------------------
temp_file.tmp                  File               18 bytes
temp_data.tmp                  File               13 bytes
```

## Grouping Examples

### 7. Group by File Extension

```bash
# Group files by extension
./target/release/lsdir examples/showcase --group-by=extension
```

**Expected Output:**

```

txt (4 files):
----------------------------------------
  small_text.txt (12 bytes)
  test_file.txt (19 bytes)
  another_test.txt (21 bytes)
  medium_doc.txt (184 bytes)
  large_file.txt (7000 bytes)

conf (1 files):
----------------------------------------
  config.conf (45 bytes)

config (1 files):
----------------------------------------
  app.config (12 bytes)

log (2 files):
----------------------------------------
  app.log (89 bytes)
  system.log (58 bytes)

py (1 files):
----------------------------------------
  main.py (137 bytes)

js (1 files):
----------------------------------------
  script.js (102 bytes)
...
```

### 8. Group by File Type

```bash
# Group by file type (File vs Directory)
./target/release/lsdir examples/showcase --group-by=file_type
```

**Expected Output:**

```

File (21 files):
----------------------------------------
  small_text.txt (12 bytes)
  test_file.txt (19 bytes)
  another_test.txt (21 bytes)
  ...
```

## Aggregation Examples

### 9. Count Files by Extension

```bash
# Count files grouped by extension
./target/release/lsdir examples/showcase --group-by=extension --function=count
```

**Expected Output:**

```
Aggregation Results:
-------------------
txt: 5
conf: 1
config: 1
log: 2
py: 1
js: 1
css: 1
csv: 1
json: 1
bin: 2
bak: 1
backup: 1
tmp: 2
```
