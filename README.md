# blatta-stream
`blatta-stream` is a TCP stream reassembly package for Rust, built on top of `smoltcp`. It allows us to read IP packets from a network interface and reassemble the TCP segments. It is similar to `pynids` in Python or `libnids` in C. Unlike the mentioned libraries, `blatta-stream` fully supports multithreading and is memory-safe.

## Installation

To use `blatta-stream` in your Rust project, you need to add the following line to your Cargo.toml file:
```
[dependencies]
    smoltcp = "0.7.5"
    blatta_stream = { git = "https://github.com/bazz-066/blatta-stream/" }
```
## Example usage
Once you have included `blatta-stream` to `Cargo.toml` file, you can start capturing the interface and reassembling TCP segments using the following code:

```
use blatta_stream::stream;
use smoltcp::phy::wait as phy_wait;
use smoltcp::phy::{Device, RawSocket, RxToken};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetFrame, PrettyPrinter, Ipv4Packet, EthernetProtocol, IpProtocol, TcpPacket};

fn main() {
    let ifname = env::args().nth(1).unwrap();
    let port_filter = Vec::from([80u16]);
    let mut srt_controller = stream::StreamReaderController::new(port_filter, false, ifname);
    
    let handle = thread::spawn(move || {
        loop {
            let ten_millis = std::time::Duration::from_millis(10);
            thread::sleep(ten_millis);

            let data_received = srt_controller.get_ready_conn();

            match data_received {
                Some(reconstructed_packets) => {
                    println!("{:?}", reconstructed_packets.get_init_tcp_message());
                },
                None => {}
            }
        }
    });

    handle.join();
}
```
To run this code, simply execute `sudo cargo run <interface name>`. Please note that the command has to be executed by root user.
