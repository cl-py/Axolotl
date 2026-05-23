#ifndef __DEFS_H
#define __DEFS_H

struct configs{
    char message[11];
};

struct PacketEvent{
    unsigned int src_ip;
    unsigned int dest_ip;
    unsigned long long timestamp;
    char protocol[];
};

#endif