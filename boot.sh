#!/bin/bash

line_start="[-]"
good_result="[0]"
bad_result="[X]"

echo "$line_start Beginning boot up process"
echo "$line_start Checking dependencies"
echo -e "==========================\n"

Packages=(
    "bpftool" 
    "build-essential"
    "cargo"
    "clang"
    "llvm"
    "make"
    "pkg-config"
    "libbpf-dev"
    "libelf-dev"
    "zlib1g-dev"
)

for package in "${Packages[@]}"; do

    if dpkg -s "$package" > /dev/null 2>&1; then
        echo "$good_result $package installed"
    else
        echo "$bad_result $package not installed."
        echo "$line_start Installing..."
        sudo apt install -y $package
    fi

done

if [ ! -f "vmlinux.h" ]; then
    echo "$bad_result vmlinux.h not found. Generating from system BTF..."
    
    if sudo bpftool btf dump file /sys/kernel/btf/vmlinux format c > vmlinux.h 2>/dev/null; then
        echo "$good_result Successfully generated vmlinux.h"
    else
        echo "$bad_result Failed to generate vmlinux.h. Ensure your kernel supports BTF."
        exit 1
    fi
else
    echo "$good_result vmlinux.h requirement met."
fi

make

sudo ./Axolotl