use drone_core::log::Port;

fn handler() {
    // Familiar output macros just work!
    println!("Hello, world!");
    dbg!(123);

    // You can send raw bytes to different ports.
    Port::new(2).write_bytes(&[1, 2, 3]);

    // If a port accessed concurrently, you can send indivisible packets up to 4
    // bytes length.
    Port::new(3).write::<u32>(0xABFF_FFCD).write::<u32>(0xDCFF_FFBA);
}
