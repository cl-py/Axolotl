#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "GPL";

#define ETH_IP_4 0x0800
#define ETH_IP_6 0x86DD

SEC("tc")
int tc_ingress(struct __sk_buff *ctx)
{
	return TC_ACT_OK;
}
