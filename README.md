# eBPF TCP Counter

This project uses eBPF to count the number of TCP packets sent by the machine.

## How it works

The project consists of two parts:

1.  **eBPF program**: This program is written in C and is attached to the `tcp_sendmsg` kernel function. Every time this function is called (i.e., a TCP packet is sent), the eBPF program increments a counter in a BPF map.
2.  **Userspace program**: This program is written in Rust. It loads the eBPF program, attaches it to the kernel function, and periodically reads the value of the counter from the BPF map. When the program is terminated (e.g. by pressing Ctrl+C), it prints the final value of the counter.

## Usage

1.  **Build the eBPF program:**
    ```bash
    make -C ebpf
    ```
2.  **Build the Rust program:**
    ```bash
    cargo build
    ```
3.  **Run the Rust program:**
    ```bash
    sudo ./target/debug/ebpf-tcp-counter
    ```
    The program will run in the background and count TCP packets. Press Ctrl+C to stop the program and print the total count.

    **Note:** You need root privileges to load eBPF programs.

## License

This project is licensed under the GPL.
