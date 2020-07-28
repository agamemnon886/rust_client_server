use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufWriter, BufRead};

fn handle_client(stream: TcpStream) {
	println!("New connection: {}",
		stream.peer_addr().expect("Failed to get peer addr"));
	println!("Client connected\n");

	let mut reader = BufReader::new(&stream);
	let mut writer = BufWriter::new(&stream);
	let mut response = String::new();

	loop {
		reader.read_line(&mut response).expect("Could not read from reader");

		if response.is_empty() {
			println!("Client disconnected");
			break;
		}

		println!("Server received [{}] bytes: {}\n",
			response.len(), response.trim());

		writer.write_all(response.as_bytes()).
			expect("Could not write to writer");
		writer.flush().expect("Could not flush writer");

		response.clear();
	}
}

fn main() {
	let listener = TcpListener::bind("0.0.0.0:3333").
		expect("Failed to listen");

	// accept connections and process them, spawning a new thread for each one
	println!("Server listening on port 3333\n");

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				thread::spawn(move|| {
					// connection succeeded
					handle_client(stream)
				});
			}

			Err(e) => {
				println!("Error: {}", e);
				/* connection failed */
			}
		}
	}

	// close the socket server
	drop(listener);
}
