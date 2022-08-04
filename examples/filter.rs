fn main() {
    println!("Libpcap version: {}", npcap_rs::version());
    let devs = npcap_rs::PCap::new().unwrap();

    let dev = devs
        .devices()
        .find(|dev| dev.desc.as_ref().unwrap() == "Realtek(R) PCI(e) Ethernet Controller");

    if let Some(dev) = dev {
        let (listener, rx) = dev.open().unwrap();
        println!("filter set: {}", listener.set_filter(&dev, "ip and tcp"));

        listener.run();

        while let Ok(pack) = rx.recv() {
            println!("{} -> {}", pack.ip_hdr.src_addr, pack.ip_hdr.dest_addr);
        }
    }
}
