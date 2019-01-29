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

fn parseCommand(input : & String, mut stack: &mut Vec<String>) -> String
{
	let mut split = input.split(" ");
	let vec: Vec<&str> = split.collect();
	let mut retVal : String = "Failed to parse command".into();
	if vec.len() > 0 {
	    println!("{:?}", vec[0]);
	    match vec[0] {
	    	"push" => {
				    	stack.push(vec[1].to_string());
				    	retVal = "Pushed to stack".into();
				    	},
			"pop" => {
				let popped = stack.pop().unwrap_or_else(||{"Nothing to pop".into()});
				retVal = format!("Popped: {}", popped);
			},
			&_ => {
				retVal = "Panic".into();
			},
	    }
	}
	retVal
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
	let vecOfBytes = &line.into_bytes();
	stream.write(vecOfBytes)?;
	Ok(())
}

fn handle_client(mut stream: TcpStream, mut stack: &mut Vec<String>) -> std::io::Result<()>  {
	let line : String = read_cmd(&mut stream);  
	let retVal : String = parseCommand(&line, &mut stack);
	write_cmd(&mut stream,retVal);

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
