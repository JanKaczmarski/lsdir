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
   git clone https://github.com/your-username/lsdir.git
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
- `-f, --function <FUNCTION>` - Aggregating function to use
- `-p, --params <PARAMS>` - Parameters for the aggregating function

### Available Fields

- `name` - File name
- `extension` - File extension
- `size` - File size in bytes
- `file_type` - File type (File or Directory)
- `modified` - Last modification time
- `accessed` - Last access time
- `created` - Creation time

### Available Operators

- `eq` / `equal` / `equals` - Equal to
- `ne` / `not_equal` / `neq` - Not equal to
- `gt` / `greater` / `greater_than` - Greater than
- `ge` / `gte` / `greater_equal` - Greater than or equal
- `lt` / `less` / `less_than` - Less than
- `le` / `lte` / `less_equal` - Less than or equal
- `contains` - Contains substring
- `starts_with` / `startswith` - Starts with
- `ends_with` / `endswith` - Ends with

### Available Functions

- `count` - Count items
- `sum` - Sum numeric values
- `avg` - Average of numeric values
- `max` - Maximum value
- `min` - Minimum value

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
lsdir --where="size,gt,1000"

# Only Rust files
lsdir --where="extension,eq,rs"

# Files containing "test" in name
lsdir --where="name,contains,test"

# Files starting with "lib"
lsdir --where="name,starts_with,lib"

# Pattern matching with wildcards
lsdir --where="name,eq,test_*"
```

### Grouping Files

```bash
# Group by file type
lsdir --group-by=file_type

# Group by file extension
lsdir --group-by=extension

# Group by size (exact byte count)
lsdir --group-by=size
```

### Aggregation Functions

```bash
# Count all files
lsdir --function=count

# Sum total size of all files
lsdir --function=sum --params=size

# Average file size
lsdir --function=avg --params=size

# Largest file size
lsdir --function=max --params=size

# Smallest file size
lsdir --function=min --params=size
```

### Complex Queries

```bash
# Count files by type, only for large files
lsdir --group-by=file_type --function=count --where="size,gt,1000"

# Sum size by extension for Rust and C files
lsdir --group-by=extension --function=sum --params=size --where="extension,contains,rs"

# Average size of files containing "main"
lsdir --function=avg --params=size --where="name,contains,main"

# Group by extension and show total size, only for files > 100 bytes
lsdir --group-by=extension --function=sum --params=size --where="size,gt,100"
```