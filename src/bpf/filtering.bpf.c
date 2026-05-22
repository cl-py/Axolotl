#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <linux/if_ether.h>

char LICENSE[] SEC("license") = "GPL";

SEC("tc")
int tc_ingress(struct __sk_buff *ctx)
{
	void *data = (void*)(long)ctx->data;
	void *data_end = (void*)(long)ctx->data_end;
	
	// IPv4 packet
	if (ctx->protocol == bpf_htons(ETH_P_IP)){
		// IPv4 Packet
		return TC_ACT_OK;
	}else if (ctx->protocol == bpf_htons(ETH_P_IPV6){
		// IPv6 Packet
		return TC_ACT_OK;
	}else{
		// All other traffic
		return TC_ACT_OK;
	}

	// src ip
	struct ethhdr *6;

	return TC_ACT_OK;
}
