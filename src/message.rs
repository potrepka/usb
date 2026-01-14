pub fn print_message(data: &[u8]) {
    let len = data.len();
    let (hex, dec, ascii) = data.iter().fold(
        (
            Vec::with_capacity(len),
            Vec::with_capacity(len),
            String::with_capacity(len),
        ),
        |(mut h, mut d, mut a), &b| {
            h.push(format!("{:02X}", b));
            d.push(format!("{:3}", b));
            a.push(if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' });
            (h, d, a)
        },
    );
    println!("Received {} bytes: {}", len, hex.join(" "));
    println!("  As decimal:   {}", dec.join(" "));
    println!("  As ASCII:     {}", ascii);
    println!();
}
