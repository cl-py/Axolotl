#include <linux/bpf.h>
#include <linux/types.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/pkt_cls.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>
#include "defs.h"

char LICENSE[] SEC("license") = "GPL";


struct{
    __uint(type, BPF_MAP_TYPE_USER_RINGBUF);
    __uint(max_entries, 256 * 1024);
} user_ring SEC(".maps");

static long user_ringbuf_callback(struct bpf_dynptr *dynptr, void *context)
{
    const struct configs *data;

    data = bpf_dynptr_data(dynptr, 0, sizeof(*data));
    if (!data)
        return 0;
    bpf_printk("PLACEHOLDER: %s",data->message);
    return 0;
}

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
		bpf_printk("got IPv4 packet from: %pI4, incoming to %pI4\n", &iph->saddr, &iph->daddr);

	}else if (ctx->protocol == bpf_htons(ETH_P_IPV6)){
		// IPv6 Packet
		return TC_ACT_OK;
	}else{
		// All other traffic
		return TC_ACT_OK;
	}

	bpf_user_ringbuf_drain(&user_ring, user_ringbuf_callback, NULL, 0);


	return TC_ACT_OK;
}
