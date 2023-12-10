
use packet::Builder;
use packet::{icmp, ip};
use socket2::{Domain, Protocol, Socket, Type};
use std::time::Instant;
use std::net::{ToSocketAddrs};
use std::env;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let url = &args[1];

    let server_details = url.to_owned()+":80";
    let payload = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let socketaddr = server_details
        .to_socket_addrs()
        .expect("Unable to resolve domain")
        .next();
    println!("*PING {} ({}) {} bytes of data", url, socketaddr.unwrap().ip(), payload.len());

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    socket.connect(&socketaddr.unwrap().into())?;

    let mut sequence = 1;
    let packets_to_send = 4;
    let mut packets_received = 0;
    let mut durations: Vec<f32> = Vec::new();

    let start_time = Instant::now();
    for _i in 1..=packets_to_send {
        // Build ICMP packet
        let builder = icmp::echo::Builder::default();
        let mut echo_buffer = builder.identifier(1).unwrap()
                                     .sequence(sequence).unwrap()
                                     .request().unwrap()
                                     .payload(payload).unwrap();

        echo_buffer.finalizer();

        let buffer = echo_buffer.build().unwrap();
        // Send data
        let sent_time = Instant::now();
        let _sent = socket.send(&buffer)?;

        loop {
            // https://github.com/rust-lang/socket2/issues/270
            let mut rec_buffer = Vec::with_capacity(4096);
            let received = socket.recv(rec_buffer.spare_capacity_mut())?;

            unsafe {
                rec_buffer.set_len(received);
            }

            let ip_parser = ip::v4::Packet::new(rec_buffer.clone()).expect("IP v4 parsing error");

            // Remove non ICMP bytes
            rec_buffer.drain(0..20);

            // Parse received buffer
            let parser = icmp::echo::Packet::new(rec_buffer).expect("Packet parsing error");

            // A reply means we have our value
            if parser.is_reply() {
                let duration = sent_time.elapsed().as_micros() as f32/1000 as f32;
                packets_received += 1;
                durations.push(duration);

                println!("{} bytes from ({}) : icmp_seq={} ttl={} duration={:.2} ms",
                    ip_parser.length(),
                    ip_parser.source(),
                    parser.sequence(),
                    ip_parser.ttl(),
                    duration);

                break;
            }
        }

        // No waiting if it is the last packet
        if _i < packets_to_send {
            let duration_sleep = Duration::from_millis(800);
            thread::sleep(duration_sleep);
        }

        sequence += 1;
    }

    let total_duration = start_time.elapsed().as_micros() as f32/1000 as f32;
    let durations_iterator = durations.clone().into_iter();
    let durations_sum: f32 = durations_iterator.clone().sum();
    let durations_count: usize = durations_iterator.clone().count();

    let loss_packets = (packets_to_send - packets_received)/packets_to_send;
    let min_duration = durations.clone().into_iter().reduce(|a, b| if a < b { return a } else { return b }).unwrap();
    let max_duration = durations.clone().into_iter().reduce(|a, b| if a > b { return a } else { return b }).unwrap();
    let avg_duration = durations_sum as f32/durations_count as f32;

    println!("*---- statistics ping {} ----", url);
    println!("{} packets transmitted, {} received, {}% packet loss, time {:.2}ms", packets_to_send, packets_received, loss_packets, total_duration);
    println!("rtt min/avg/max/mdev = {:.3}/{:.3}/{:.3}/0 ms", min_duration, avg_duration, max_duration);

    Ok(())
}