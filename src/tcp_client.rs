use std::{
    io::{self, BufRead, Write},
    net::TcpStream,
    str,
};

use std::io::BufReader;

pub fn connect(address: &str) -> Result<(), failure::Error> {
    let mut stream = TcpStream::connect(address)?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        print!("{}", str::from_utf8(&buffer)?);
        print!("{}", str::from_utf8(&buffer)?);
    }
}
