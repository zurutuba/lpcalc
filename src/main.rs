
use std::io;
use std::io::Write;

mod calc;

fn main() {
    let mut inputstr = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut inputstr) {
            Err(e) => println!("Couldn't read line: {}", e),
            Ok(_) => {
                // Better way to do this?
                if inputstr == "\r\n" {
                    break;
                } else {
                    match calc::calculate(&inputstr) {
                        Ok(calc::Tokens::Number(a)) => println!("= {}", a),
                        Ok(_)  => println!("Invalid result."),
                        Err(e) => println!("Error: {:?}", e),
                    };
                } 
            }
        }

        inputstr = "".to_owned();
    }
}
