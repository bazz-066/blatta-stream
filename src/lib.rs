pub mod stream;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let ifname = std::env::args().nth(1).unwrap();
        let port_filter = Vec::from([80u16]);
        let mut srt_controller = crate::stream::StreamReaderController::new(port_filter, false, ifname);

        let handle = std::thread::spawn(move || {
            loop {
                let ten_millis = std::time::Duration::from_millis(10);
                std::thread::sleep(ten_millis);

                let data_received = srt_controller.get_ready_conn();

                match data_received {
                    Some(reconstructed_packets) => {
                        let msg_len = reconstructed_packets.get_init_tcp_message().len();
                        assert!(msg_len > 0);
                        break;
                    },
                    None => {}
                }
            }
        });

        handle.join();
    }
}
