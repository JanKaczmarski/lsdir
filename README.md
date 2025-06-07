# lsdir - SQL-like File Directory Analysis Tool

A Rust CLI application that performs SQL-like operations (grouping, filtering, aggregating) on files in a directory. Think of it as `ls` meets SQL for powerful file system analysis.

## Features

- **SQL-like syntax** for file operations
- **Filtering** with WHERE clauses supporting various operators
- **Grouping** by file attributes (type, extension, size, etc.)
- **Aggregation functions** (COUNT, SUM, AVG, MAX, MIN)
- **Pattern matching** with wildcards
- **Cross-platform** support (Linux, macOS, Windows)

## Installation


### Prerequisites

- **Rust** (1.70.0 or later)
- **Cargo** (comes with Rust)
- **Git** (for cloning the repository)

### Installation Methods

#### Method 1: Build from Source (Recommended)

1. **Clone the repository:**

   ```bash
   git clone https://github.com/JanKaczmarski/lsdir.git
   cd lsdir
   ```

2. **Build the project:**

   ```bash
   # For development (faster compilation)
   cargo build
   
   # For production (optimized binary)
   cargo build --release
   ```

3. **Run the tool:**

   ```bash
   # Development build
   ./target/debug/lsdir --help
   
   # Release build
   ./target/release/lsdir --help
   ```

4. **Optional: Install globally:**

   ```bash
   cargo install --path .
   # Now you can use 'lsdir' from anywhere
   ```

### Quick Setup Verification

1. **Build the project:**

   ```bash
   cargo build
   ```

2. **Create sample files:**

   ```bash
   cd examples/showcase
   ../create_samples.sh
   cd ../..
   ```

3. **Run basic test:**

   ```bash
   ./target/debug/lsdir examples/showcase
   ```

4. **Run the demo:**

   ```bash
   ./examples/demo.sh
   ```

## Usage

```bash
lsdir [OPTIONS] [PATH]
```

### Arguments

- `[PATH]` - Directory path to analyze (defaults to current directory)

### Options

- `-g, --group-by <FIELD>` - GROUP BY clause - field to group files by
- `-w, --where <CONDITION>` - WHERE clause - filter condition in format: field,operator,value
- `-a, --aggregate <FUNCTION>` - Aggregating function to use

### Available Fields

- `name` / `n` - File name (regex)
- `extension` / `ext` / `e` - File extension
- `size` / `s` - File size in bytes
- `file_type` / `type` / `f` / `t` - File type (File or Directory)
- `modified` / `mod` / `m` - Last modification time
- `accessed` / `acc` / `a` - Last access time
- `created` / `cre` / `c` - Creation time

### Available Operators for size and dates

- `eq` / `equal` / `equals` - Equal to
- `ne` / `not_equal` / `neq` - Not equal to
- `gt` / `greater` / `greater_than` - Greater than
- `ge` / `gte` / `greater_equal` - Greater than or equal
- `lt` / `less` / `less_than` - Less than
- `le` / `lte` / `less_equal` - Less than or equal

### Available Aggregation Functions

- `count` / `c` - Count items
- `sum` / `s` - Sum numeric values
- `avg` / `a` - Average of numeric values
- `max` - Maximum value
- `min` - Minimum value

### Available Grouping for size
- `bytes` / `b` - Group by exact byte size
- `kilobytes` / `kb` - Group by kilobytes (1024 bytes)
- `megabytes` / `mb` - Group by megabytes (1024 kilobytes)
- `gigabytes` / `gb` - Group by gigabytes (1024 megabytes)
- `terabytes` / `tb` - Group by terabytes (1024 gigabytes)

### Available Grouping for dates
- `second` / `sec` / `s` - Group by seconds
- `minute` / `min` - Group by minutes
- `hour` / `h` - Group by hours
- `day` / `d` - Group by days
- `week` / `w` - Group by weeks
- `month` / `m` - Group by months
- `year` / `y` - Group by years

## Examples

### Basic File Listing

```bash
# List all files in current directory
lsdir

# List files in a specific directory
lsdir /path/to/directory
```

### Filtering with WHERE Clauses

```bash
# Files larger than 1000 bytes
lsdir --where=size,gt,1000
lsdir -w=s,gt,1000

# Only Rust files
lsdir --where=extension,rs
lsdir -w=e,rs

# Files containing "test" in name
lsdir --where=name,test
lsdir -w=n,test


# Pattern matching with wildcards
lsdir --where=name,test*.txt
lsdir -w=n,test*.txt

```

### Grouping Files

```bash
# Group by file type
lsdir --group-by=file_type
lsdir -g=f


# Group by file extension
lsdir --group-by=extension
lsdir -g=e


# Group by size (exact byte count)
lsdir --group-by=size,bytes
lsdir -g=s,b
```

### Aggregation Functions

```bash
# Count all files
lsdir --aggregate=count
lsdir -a=c

# Sum total size of all files
lsdir --aggregate=sum,size
lsdir -a=s,s

# Average file size
lsdir --aggregate=avg,size
lsdir -a=a,s

# Largest file size
lsdir --function=max,size
lsdir --function=max,s

# Smallest file size
lsdir --function=min,size
lsdir --function=min,s
```

### Complex Queries

```bash
# Count files by type, only for large files
lsdir --group-by=file_type --aggregate=count --where=size,gt,1000
lsdir -g=f -a=c -w=s,gt,1000

# Sum size by extension for Rust files
lsdir --group-by=extension --aggregate=sum,size --where=extension,rs
lsdir -g=e -a=s,s -w=e,rs

# Average size of files containing "main"
lsdir --aggregate=avg,size --where=name,main
lsdir -a=a,s -w=n,main

# Group by extension and show total size, only for files > 100 bytes
lsdir --group-by=extension --aggregate=sum,size --where=size,gt,100
lsdir -g=e -a=s,s -w=s,gt,100
```