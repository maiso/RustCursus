use std::io::prelude::*;
use std::io::{Read, Write };
use std::net::{TcpListener, TcpStream};
use std::io::{self, BufReader};

enum MyCommand{
	Push{
		item: String,
	},
	Pop,
	InvalidCommand {reason: String,},
	InvalidParameter{reason: String,},
}

fn parse_command(input : & String) -> Result<MyCommand,()> {
	let mut split = input.split(" ");
	let command = split.next().unwrap_or("none");

    println!("Command: {:?}", command);
    match command {
    	"push"  => {
    		let push_items = split.next().unwrap_or("none");
    		if push_items != "none"{
    			Ok(MyCommand::Push{item: push_items.to_string()})
    		}else{
    			Ok(MyCommand::InvalidParameter{ reason: format!("Nothing to push")})
    		}},
		"pop"  => Ok(MyCommand::Pop),
		&_     => Ok(MyCommand::InvalidCommand { reason: format!("Unknown Command : {:?}",command)}),
    }
}

fn read_cmd(stream : &mut TcpStream) -> Result<String,std::io::Error>
{
 	let mut f = BufReader::new(stream);

	let mut buf = String::new();

    f.read_line(&mut buf).expect("reading from cursor won't fail"); 
	println!("Read: {:?}", buf);
	buf.truncate(buf.len() - 2); //Remove the \r\n

    Ok(buf)
}

fn write_cmd( stream : &mut TcpStream, line : String) -> std::io::Result<()> {
	println!("Write {:?}",line);
	let vec_of_bytes = &line.into_bytes();
	stream.write(vec_of_bytes)?;
	Ok(())
}

fn handle_received_command(my_command : &MyCommand, stack: &mut Vec<String>) -> String{
	match my_command {
		MyCommand::Pop => {
			println!("Pop");
			let popped = stack.pop().unwrap_or_else(||{"Nothing to pop".into()});
			return format!("Popped: {}", popped);
		},
		MyCommand::Push{item : x} => {
			println!("Push {}",x);
	    	stack.push(x.to_string());
	    	return "Pushed to stack".into();
		},
		MyCommand::InvalidCommand{reason : x} => {
	    	return "InvalidCommand: ".into();
		},	
		MyCommand::InvalidParameter{reason : x} => {
	    	return "InvalidParameter: ".into();
		},		
	}	
}
fn handle_client(mut stream: TcpStream, stack: &mut Vec<String>) -> std::io::Result<()>  {
	let line : String = read_cmd(&mut stream)?;  
	let mut ret_val : String = "retval".into();

	match parse_command(&line){
        Ok(my_command) => { ret_val = handle_received_command(&my_command,stack); },
        Err(_) => {},
    }

	write_cmd(&mut stream,ret_val)?;

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

#[cfg(test)]
mod tests{
	use proptest::*;
	use super::*;

	proptest!{
		#[test]
		fn doesnt_crash(ref input in "(push){0,3}(pop){0,3}.*"){
			let _ = parse_command(input);
		}
	}
}

