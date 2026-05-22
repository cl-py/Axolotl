#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <linux/if_ether.h>

char LICENSE[] SEC("license") = "GPL";

SEC("tc")
int tc_ingress(struct __sk_buff *ctx)
{
	void *data = (void*)(long)ctx->data;
	void *data_end = (void*)(long)ctx->data_end;
	
	// src ip
	struct ethhdr *6;

	return TC_ACT_OK;
}
