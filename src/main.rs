use std::io::prelude::*;
use std::io::{Read, Write };
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufReader};

enum MyCommand{
	Push{
		item: String,
	},
	Pop
}

enum MyError{
	//Io(std::io::Error),
	Parse {
		reason: String,
	},
}

fn parse_command(input : & String) -> Result<MyCommand,MyError> {
	let split = input.split(" ");
	let vec: Vec<&str> = split.collect();

	if vec.len() > 0 {
	    println!("{:?}", vec[0]);
	    match vec[0] {
	    	"push" => Ok(MyCommand::Push{item: vec[1].to_string()}),
			"pop" =>  Ok(MyCommand::Pop),
			&_ => Err(MyError::Parse{ reason: format!("Invalid command {:?}",vec[0])})
	    }
	  	
	}else{
		Err(MyError::Parse{ reason:"No command provided".into()})
	}
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

fn handle_client(mut stream: TcpStream, stack: &mut Vec<String>) -> std::io::Result<()>  {
	let line : String = read_cmd(&mut stream);  
	let mut ret_val : String = "retval".into();
	match parse_command(&line){
        Ok(my_command) => {
			match my_command {
				MyCommand::Pop => {
					println!("Pop");
					let popped = stack.pop().unwrap_or_else(||{"Nothing to pop".into()});
					ret_val = format!("Popped: {}", popped);
				},
				MyCommand::Push{item : x} => {
					println!("Push {}",x);
			    	stack.push(x);
			    	ret_val = "Pushed to stack".into();
				},
			}
        	
        },
        Err(err) => {
        	match err {
        		MyError::Parse{ reason: x} => println!("Error reason: {}", x),
        		//MyError::io => println!("Error reason: io"),
        	}
        }
        ,
    }

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
