#include <linux/bpf.h>
#include <linux/types.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/pkt_cls.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>

struct {
	__uint(type, BPF_MAP_TYPE_RINGBUF);
	__uint(max_entries, 256 * 1024);
} events SEC(".maps");

char LICENSE[] SEC("license") = "GPL";

/// @tchook {"ifindex":3, "attach_point":"BPF_TC_INGRESS"}
/// @tcopts {"handle":1, "priority":1}

SEC("tc")
int tc_ingress(struct __sk_buff *ctx)
{
	void *data = (void*)(long)ctx->data;
	void *data_end = (void*)(long)ctx->data_end;

	struct ethhdr *eth;
	struct iphdr *iph;

	bpf_printk("done~");

	if (ctx->protocol == bpf_htons(ETH_P_IP)){
		eth = data;
		if ((void*)(eth + 1) > data_end)
			return TC_ACT_OK;
		iph = (struct iphdr*)(eth + 1);

		if ((void*)(iph + 1) > data_end)
			return TC_ACT_OK;
		bpf_printk("got IPv4 packet from: %d, incoming to %d\n", iph->saddr, iph->daddr);

	}else if (ctx->protocol == bpf_htons(ETH_P_IPV6)){
		// IPv6 Packet
		return TC_ACT_OK;
	}else{
		// All other traffic
		return TC_ACT_OK;
	}

	bpf_printk("done~");


	return TC_ACT_OK;
}
