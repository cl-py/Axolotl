<div align="center">
    <img src="./.public/Axolotl.svg" width=900 height=300>
</div>

--- 
<div align="center">

![C](https://img.shields.io/badge/c-%2300599C.svg?style=for-the-badge&logo=c&logoColor=white)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Bash Script](https://img.shields.io/badge/bash_script-%23121011.svg?style=for-the-badge&logo=gnu-bash&logoColor=white)
![Openwrt](https://img.shields.io/badge/OpenWRT-00B5E2?style=for-the-badge&logo=OpenWrt&logoColor=white)
![Raspberry Pi](https://img.shields.io/badge/-Raspberry_Pi-C51A4A?style=for-the-badge&logo=Raspberry-Pi)

</div>

---

## What is Axolotl?

Axolotl consists of a series of eBPF programs that implement an enhanced firewall and traffic monitoring system onto OpenWRT. This was built with eBPF programs made in C that were loaded onto the OpenWRT programs and communicate with userspace code written in Rust. The OpenWRT, a Linux based Operating System used for routers, was loaded onto the Raspberry Pi 5 where it had the eBPF programs attached to it.  

eBPF (extended Berkeley Packet Filter) is a technology, originating in the Linux Kernel, that allows for programs to run with elevated privileges inside kernel space. They are event-driven programs that hook into the kernel similar to kprobes.  

To learn more information on eBPF visit: https://ebpf.io/  
To learn more about OpenWRT visit: https://openwrt.org/  


## Features

- Firewall ruleset configuration  
- Packet Monitoring 

## How to use

To begin using run the following on a *Linux Device*

```
chmod +x boot.sh
./boot.sh
```

Make sure to allow **all** installs for dependencies that follow

## Contributors
[![Claudia](https://img.shields.io/badge/Claudia_Varnas-FFA6D7?style=for-the-badge&logo=github&logoColor=white)](https://github.com/cl-py)
[![Eduardo](https://img.shields.io/badge/Eduardo_Murillo-F9C2E2?style=for-the-badge&logo=github&logoColor=white)](https://github.com/edmuri)
