/**
 * @file filtering.bpf.c
 * 
 * @brief bpf prog for filtering based off ip
 * 
 * @details
 */

#include <linux/bpf.h>
#include <linux/types.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/pkt_cls.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>

#include "defs.h"


char LICENSE[] SEC("license") = "GPL";

/**==========================================================================================*
 * 										 	 BPF Data	   								     *
 *===========================================================================================*/

struct{											//	User ring takes data from user space.
    __uint(type, BPF_MAP_TYPE_USER_RINGBUF);	//	This is used to read the configurations
    __uint(max_entries, 256 * 1024);			//  set by the client.
} user_ring SEC(".maps");						//

struct {										//
	__uint(type, BPF_MAP_TYPE_RINGBUF);			//
	__uint(max_entries, 256 * 1024);			//
} events SEC(".maps");							//

/**==========================================================================================*
 * 										 	 Code	   									     *
 *===========================================================================================*/

/**
 * @brief User ring callback function when draining
 * 
 * @param dynptr
 * @param context
 * @return 0 on success, 1 otherwise
 * 
 * @details This function allows us to read data from userspace program. Used specifically for the ip configurations.
 * 
 */
static long user_ringbuf_callback(struct bpf_dynptr *dynptr, void *context)
{
    const struct configs *data;

    data = bpf_dynptr_data(dynptr, 0, sizeof(*data));
    if (!data)
        return 0;
    bpf_printk("PLACEHOLDER: %s",data->message);
    return 0;
}

/**
 * @brief Hook for the traffic control later
 * 
 * @param ctx -> socket buffer data
 * @return Whether or not to allow the packet to proceed
 * 
 * @details	This is the hook for the traffic control layer. This is the earliest we hook into the network stack
 * 			allowing us to filter our IPs without the overhead of the rest of the stack. 
 */
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
		// can be viewed at /sys/kernel/tracing/trace_pipe
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
