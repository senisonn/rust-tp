use tokio::net::UdpSocket;
use std::collections::HashMap;
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:5354").await?;
    println!("Serveur DNS écoute sur 127.0.0.1:5354");

    let records = HashMap::from([
        ("example.com".to_string(), Ipv4Addr::new(192, 168, 1, 1)),
        ("test.local".to_string(), Ipv4Addr::new(10, 0, 0, 1)),
    ]);

    let mut buf = [0u8; 512];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Reçu {} octets de {}", len, addr);

        if let Some((tx_id, qname)) = parse_query(&buf[..len]) {
            println!("Requête DNS pour: {}", qname);

            if let Some(ip) = records.get(&qname) {
                let response = build_response(tx_id, &qname, *ip);
                socket.send_to(&response, addr).await?;
            } else {
                println!("Nom inconnu: {}", qname);
            }
        }
    }
}


fn parse_query(buf: &[u8]) -> Option<(u16, String)> {
    if buf.len() < 12 {
        return None;
    }

    let id = u16::from_be_bytes([buf[0], buf[1]]);
    let qdcount = u16::from_be_bytes([buf[4], buf[5]]);
    if qdcount == 0 {
        return None;
    }

    let mut pos = 12;
    let mut labels = Vec::new();

    while pos < buf.len() {
        let len = buf[pos] as usize;
        if len == 0 {
            break;
        }

        pos += 1;
        if pos + len > buf.len() {
            return None;
        }

        let label = String::from_utf8_lossy(&buf[pos..pos+len]).to_string();
        labels.push(label);
        pos += len;
    }

    let qname = labels.join(".");
    Some((id, qname))
}


fn build_response(id: u16, qname: &str, ip: Ipv4Addr) -> Vec<u8> {
    let mut packet = Vec::new();

    // Header
    packet.extend(&id.to_be_bytes());        // ID
    packet.extend(&0x8180u16.to_be_bytes()); // Flags (standard response, no error)
    packet.extend(&1u16.to_be_bytes());      // QDCOUNT
    packet.extend(&1u16.to_be_bytes());      // ANCOUNT
    packet.extend(&0u16.to_be_bytes());      // NSCOUNT
    packet.extend(&0u16.to_be_bytes());      // ARCOUNT

    // Question section
    for label in qname.split('.') {
        packet.push(label.len() as u8);
        packet.extend(label.as_bytes());
    }
    packet.push(0);                           // End of QNAME
    packet.extend(&1u16.to_be_bytes());      // QTYPE A
    packet.extend(&1u16.to_be_bytes());      // QCLASS IN

    // Answer section
    // Use pointer to offset 12 (0xC00C)
    packet.extend(&0xC00Cu16.to_be_bytes()); // NAME pointer to question
    packet.extend(&1u16.to_be_bytes());      // TYPE A
    packet.extend(&1u16.to_be_bytes());      // CLASS IN
    packet.extend(&300u32.to_be_bytes());    // TTL
    packet.extend(&4u16.to_be_bytes());      // RDLENGTH
    packet.extend(&ip.octets());             // RDATA (IPv4 addr)

    packet
}
