OUTPUT := .output
CLANG ?= clang
CARGO ?= cargo
TARGET_DIR := target/release
BINARY_NAME := Axolotl
BPF_NAME := logging
BPFTOOL_OUTPUT ?= $(abspath $(OUTPUT)/bpftool)
# BPFTOOL ?= bpftool
BPFTOOL ?= /usr/sbin/bpftool
ARCH ?= $(shell uname -m | sed 's/x86_64/x86/' \
			 | sed 's/arm.*/arm/' \
			 | sed 's/aarch64/arm64/' \
			 | sed 's/ppc64le/powerpc/' \
			 | sed 's/mips.*/mips/' \
			 | sed 's/riscv64/riscv/' \
			 | sed 's/loongarch64/loongarch/')
VMLINUX := ./vmlinux.h

# Get Clang's default includes on this system
CLANG_BPF_SYS_INCLUDES ?= $(shell $(CLANG) -v -E - </dev/null 2>&1 \
	| sed -n '/<...> search starts here:/,/End of search list./{ s| \(/.*\)|-idirafter \1|p }')

INCLUDES := -I$(OUTPUT) -I.

ifeq ($(V),1)
	Q =
	msg =
else
	Q = @
	msg = @printf '  %-8s %s%s\n'					\
		      "$(1)"						\
		      "$(patsubst $(abspath $(OUTPUT))/%,%,$(2))"	\
		      "$(if $(3), $(3))";
	MAKEFLAGS += --no-print-directory
endif

.PHONY: all
all: $(BINARY_NAME)

.PHONY: clean
clean:
	$(call msg,CLEAN)
	$(Q)rm -rf $(OUTPUT) $(TARGET_DIR) $(BINARY_NAME)
	$(Q)$(CARGO) clean

$(OUTPUT) $(OUTPUT)/libbpf $(BPFTOOL_OUTPUT):
	$(call msg,MKDIR,$@)
	$(Q)mkdir -p $@

# Build BPF object files from C source
$(OUTPUT)/%.bpf.o: src/bpf/%.bpf.c $(VMLINUX) | $(OUTPUT)
	$(call msg,BPF,$@)
	$(Q)$(CLANG) -g -O2 -target bpf -D__TARGET_ARCH_$(ARCH)		      \
		     $(INCLUDES) $(CLANG_BPF_SYS_INCLUDES)		      \
		     -c $< -o $(patsubst %.bpf.o,%.tmp.bpf.o,$@)
	$(Q)$(BPFTOOL) gen object $@ $(patsubst %.bpf.o,%.tmp.bpf.o,$@)

# Generate BPF skeletons (if needed for Rust)
$(OUTPUT)/%.skel.h: $(BPF_NAME)/%.bpf.o | $(OUTPUT)
	$(call msg,GEN-SKEL,$@)
	$(Q)$(BPFTOOL) gen skeleton $< > $@

# Build the Rust binary
$(BINARY_NAME): $(OUTPUT)/$(BPF_NAME).bpf.o | $(OUTPUT)
	$(call msg,CARGO,$@)
	$(Q)$(CARGO) build --release
	$(Q)cp $(TARGET_DIR)/$(BINARY_NAME) ./

# Build in debug mode
.PHONY: debug
debug: $(OUTPUT)/$(BPF_NAME).bpf.o | $(OUTPUT)
	$(call msg,CARGO,debug)
	$(Q)$(CARGO) build
	$(Q)cp target/debug/$(BINARY_NAME) ./$(BINARY_NAME)-debug

# Run cargo check
.PHONY: check
check: $(OUTPUT)/$(BPF_NAME).bpf.o | $(OUTPUT)
	$(call msg,CARGO,check)
	$(Q)$(CARGO) check

# Run cargo test
.PHONY: test
test: $(OUTPUT)/$(BPF_NAME).bpf.o | $(OUTPUT)
	$(call msg,CARGO,test)
	$(Q)$(CARGO) test

# delete failed targets
.DELETE_ON_ERROR:

# keep intermediate (.skel.h, .bpf.o, etc) targets
.SECONDARY:
