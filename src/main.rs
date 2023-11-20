use std::{net::TcpStream, io::{Write, Read, Cursor, self}};
use byteorder::{ReadBytesExt, BigEndian};

fn main() {
    let mut alphanumeric: Vec<char> = vec![];
    alphanumeric.extend(('a'..='z').into_iter());
    alphanumeric.extend(('A'..='Z').into_iter());
    alphanumeric.extend(('0'..='9').into_iter());
    alphanumeric.extend(('!'..='/').into_iter());

    println!();
    for count in 0..16 {
        if recursion("", &alphanumeric, count) {
            break;
        }
    }
}

fn recursion(add: &str, letters: &Vec<char>, limit: i32) -> bool {
    if limit == 0 {
        return false;
    }
    
    for letter in letters {
        let password = format!("{}{}", add, letter);

        print!("\rtrying... [{}]", password);
        io::stdout().flush().unwrap();

        if try_password(&password) {
            println!("found password: {}", password);
            return true;
        }
    }

    for letter in letters {
        let password = format!("{}{}", add, letter);
        if recursion(&password, &letters, limit - 1) {
            return true;
        }
    }

    return false;
}

fn try_password(password: &String) -> bool {
    let mut connection = TcpStream::connect("localhost:25575").unwrap();

    let rcon_request_id: i32 = 0;
    let rcon_type: i32 = 3;
    let payload: &str = &format!("{password}\0");

    let mut buffer = vec![];
    buffer.extend(rcon_request_id.to_le_bytes());
    buffer.extend(rcon_type.to_le_bytes());
    buffer.extend(payload.as_bytes());
    buffer.push(0);

    let mut packet_buffer: Vec<u8> = vec![];
    packet_buffer.extend((buffer.len() as i32).to_le_bytes());
    packet_buffer.extend(buffer);
    connection.write_all(&packet_buffer).unwrap();

    //println!("sent");

    let mut packet_size = vec![0; 1];
    connection.read_exact(&mut packet_size).unwrap();
    let size = packet_size[0] as usize;

    let mut response_buffer = vec![0; size];
    connection.read_exact(&mut response_buffer).unwrap();
    let mut buffer_reader = Cursor::new(response_buffer);
    
    //println!("{:?}", buffer_reader);
    
    let _response_id = buffer_reader.read_i32::<BigEndian>().unwrap();
    let response_type = buffer_reader.read_i32::<BigEndian>().unwrap();

    //println!("id: {}, type: {}", response_id, response_type);
    return response_type == 2;
}
