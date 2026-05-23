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

struct configs{
    char message[11];
};

static long user_ringbuf_callback(struct bpf_dynptr *dynptr, void *context)
{
    const struct configs *data;

    data = bpf_dynptr_data(dynptr, 0, sizeof(*data));
    if (!data)
        return 0;
    bpf_printk("Hello From user-space: %s",data->message);
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_execve")
int handle_execve(void *ctx) {
    // char *message = bpf_ringbuf_reserve(&events, 13, 0);
    // if (!message) {
    //     return 0;
    // }

    // __builtin_memcpy(message, "Hello World!", 13);
    // bpf_ringbuf_submit(message, 0);
    bpf_user_ringbuf_drain(&user_ring, user_ringbuf_callback, NULL, 0);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";