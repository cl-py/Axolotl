#!/bin/bash

install_dependencies() {

    echo "$line_start Checking dependencies"
    echo -e "==========================\n"

    Packages=(
        "linux-tools-common"
        "linux-tools-$(uname -r)" 
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
            apt install -y $package
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

}

verify_root(){

    if [ "$EUID" -ne 0 ]; then
        echo "$bad_result Please run with root permissions"
        exit 1
    fi

}

#######################################################################

line_start="[-]"
good_result="[0]"
bad_result="[X]"

verify_root

echo "$line_start Beginning boot up process"

install_dependencies

make

#######################################################################