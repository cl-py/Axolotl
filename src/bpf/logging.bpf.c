#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} events SEC(".maps");

struct{
    __uint(type, BPF_MAP_TYPE_USER_RINGBUF);
    __uint(max_entries, 256 * 1024);
} user_ring SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_execve")
int handle_execve(void *ctx) {
    char *message = bpf_ringbuf_reserve(&events, 13, 0);
    if (!message) {
        return 0;
    }

    __builtin_memcpy(message, "Hello World!", 13);
    bpf_ringbuf_submit(message, 0);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";