#!/bin/bash
# lsdir Demo Script
# This script demonstrates various features of the lsdir tool

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored headers
print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

print_command() {
    echo -e "${YELLOW}Command: $1${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Check if lsdir binary exists
if [ ! -f "./target/release/lsdir" ] && [ ! -f "./target/debug/lsdir" ]; then
    echo -e "${RED}Error: lsdir binary not found. Please run 'cargo build' first.${NC}"
    exit 1
fi

# Use debug build if release not available
LSDIR="./target/release/lsdir"
if [ ! -f "$LSDIR" ]; then
    LSDIR="./target/debug/lsdir"
fi

# Create examples if they don't exist
if [ ! -d "examples/showcase" ]; then
    echo "Creating example files..."
    mkdir -p examples/showcase
    cd examples/showcase
    ../create_samples.sh > /dev/null 2>&1
    cd ../..
fi

echo -e "${GREEN}🚀 lsdir Tool Demonstration${NC}"
echo "This demo showcases the SQL-like file analysis capabilities of lsdir"

# Basic listing
print_header "1. Basic File Listing"
print_command "$LSDIR examples/showcase"
$LSDIR examples/showcase
print_success "Listed all files with their types and sizes"

# Counting
print_header "2. Count Total Files"
print_command "$LSDIR examples/showcase --function=count"
$LSDIR examples/showcase --function=count
print_success "Counted total number of files"

# Size filtering
print_header "3. Filter Large Files (>1000 bytes)"
print_command "$LSDIR examples/showcase --where=\"size,gt,1000\""
$LSDIR examples/showcase --where="size,gt,1000"
print_success "Filtered files by size"

# Extension filtering
print_header "4. Filter Text Files"
print_command "$LSDIR examples/showcase --where=\"extension,eq,txt\""
$LSDIR examples/showcase --where="extension,eq,txt"
print_success "Filtered files by extension"

# Pattern matching
print_header "5. Pattern Matching (files containing 'test')"
print_command "$LSDIR examples/showcase --where=\"name,contains,test\""
$LSDIR examples/showcase --where="name,contains,test"
print_success "Used pattern matching to find files"

# Grouping by extension
print_header "6. Group Files by Extension"
print_command "$LSDIR examples/showcase --group-by=extension"
$LSDIR examples/showcase --group-by=extension
print_success "Grouped files by their extensions"

# Count by extension
print_header "7. Count Files by Extension"
print_command "$LSDIR examples/showcase --group-by=extension --function=count"
$LSDIR examples/showcase --group-by=extension --function=count
print_success "Counted files grouped by extension"

# Sum sizes by extension
print_header "8. Total Size by Extension"
print_command "$LSDIR examples/showcase --group-by=extension --function=sum --params=size"
$LSDIR examples/showcase --group-by=extension --function=sum --params=size
print_success "Calculated total size by extension"

# Average size
print_header "9. Average File Size"
print_command "$LSDIR examples/showcase --function=avg --params=size"
$LSDIR examples/showcase --function=avg --params=size
print_success "Calculated average file size"

# Complex query
print_header "10. Complex Query: Large Files by Extension"
print_command "$LSDIR examples/showcase --where=\"size,gt,100\" --group-by=extension --function=count"
$LSDIR examples/showcase --where="size,gt,100" --group-by=extension --function=count
print_success "Combined filtering, grouping, and aggregation"

# Wildcard matching
print_header "11. Wildcard Pattern Matching"
print_command "$LSDIR examples/showcase --where=\"name,eq,temp_*\""
$LSDIR examples/showcase --where="name,eq,temp_*"
print_success "Used wildcard patterns for flexible matching"

# Maximum file size
print_header "12. Find Largest Files by Extension"
print_command "$LSDIR examples/showcase --group-by=extension --function=max --params=size"
$LSDIR examples/showcase --group-by=extension --function=max --params=size
print_success "Found largest files in each extension group"

echo -e "\n${GREEN}🎉 Demo Complete!${NC}"
echo -e "You've seen how lsdir can:"
echo -e "  ✓ List and count files"
echo -e "  ✓ Filter by size, extension, and name patterns"
echo -e "  ✓ Group files by various attributes"
echo -e "  ✓ Calculate aggregations (count, sum, avg, max, min)"
echo -e "  ✓ Combine multiple operations for complex analysis"
echo ""
echo -e "For more examples, see: ${BLUE}examples/EXAMPLES.md${NC}"
echo -e "For full documentation, see: ${BLUE}README.md${NC}"
