use std::net::{TcpStream};
use std::io::{Write, stdin, stdout, BufReader, BufWriter, BufRead};

fn main() {
	let stream = TcpStream::connect("localhost:3333").
		expect("Failed to connect");

	println!("Successfully connected to server in port 3333\n");

	let mut writer = BufWriter::new(&stream);
	let mut reader = BufReader::new(&stream);
	let mut response = String::new();
	let mut msg = String::new();

	loop {
		print!("Input text: ");
		stdout().flush().expect("Failed to read text from console");

		stdin().read_line(&mut msg).expect("Failed to read from STDIN");

		writer.write_all(msg.as_bytes()).expect("Could not write to writer");
		writer.flush().expect("Could not flush writer");

		reader.read_line(&mut response).expect("Could not read from reader");

		println!("Client received [{}] bytes: {}\n",
			response.len(), response.trim());

		response.clear();
		msg.clear();
	}
}

