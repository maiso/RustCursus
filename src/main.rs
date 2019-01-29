use std::io::prelude::*;
use std::io::{Result, Read, Write };
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufReader};

enum Command{
	Push{
		itesm: Vec<String>,
	}
}

enum Error{
	Io(std::io::Error),
	Parse {
		reason: String,
	},
}

fn parse_command(input : & String, stack: &mut Vec<String>) -> String
{
	let split = input.split(" ");
	let vec: Vec<&str> = split.collect();
	let mut ret_val : String = "Failed to parse command".into();
	if vec.len() > 0 {
	    println!("{:?}", vec[0]);
	    match vec[0] {
	    	"push" => {
				    	stack.push(vec[1].to_string());
				    	ret_val = "Pushed to stack".into();
				    	},
			"pop" => {
				let popped = stack.pop().unwrap_or_else(||{"Nothing to pop".into()});
				ret_val = format!("Popped: {}", popped);
			},
			&_ => {
				ret_val = "Invalid command".into();
			},
	    }
	}
	ret_val //return value
}

fn read_cmd(stream : &mut TcpStream) -> String
{
 	let mut f = BufReader::new(stream);

	let mut buf = String::new();

    f.read_line(&mut buf).expect("reading from cursor won't fail"); 
	println!("Read: {:?}", buf);
	buf.truncate(buf.len() - 2); //Remove the \r\n

    buf
}

fn write_cmd( stream : &mut TcpStream, line : String) -> std::io::Result<()> {
	println!("Write {:?}",line);
	let vec_of_bytes = &line.into_bytes();
	stream.write(vec_of_bytes)?;
	Ok(())
}

fn handle_client(mut stream: TcpStream, mut stack: &mut Vec<String>) -> std::io::Result<()>  {
	let line : String = read_cmd(&mut stream);  
	let ret_val : String = parse_command(&line, &mut stack);
	write_cmd(&mut stream,ret_val);

   	Ok(())
}

fn main() -> std::io::Result<()>{
	let listener = TcpListener::bind("127.0.0.1:8000").expect("Should be able to bind");

	let mut stack : Vec<String> = vec![];
	for socket in listener.incoming() {
   		handle_client(socket?,&mut stack)?;
	}
	Ok(())
}
