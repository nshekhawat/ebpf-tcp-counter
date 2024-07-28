use aya::programs::KProbe;
use aya::maps::Array;
use aya::{Bpf, Pod};
use std::convert::TryFrom;
use std::convert::TryInto;
use tokio::signal;

#[derive(Clone, Copy)]
struct Key(u32);

unsafe impl Pod for Key {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpf = Bpf::load_file("/home/nshekhawat.linux/ebpf-tcp-counter/ebpf/tcp_counter.o")?;

    let program: &mut KProbe = bpf.program_mut("bpf_prog1")
        .ok_or("Program bpf_prog1 not found")?
        .try_into()?;
    program.load()?;
    program.attach("tcp_sendmsg", 0)?;

    let map: Array<_, u64> = Array::try_from(bpf.map_mut("packet_count").ok_or("Map packet_count not found")?)?;

    // Gracefully handle termination signals
    signal::ctrl_c().await?;
    println!("Exiting...");

    let key = 0;
    match map.get(&key, 0) {
        Ok(value) => println!("TCP packet count: {}", value),
        Err(e) => println!("Error accessing the map: {}", e),
    }

    Ok(())
}
