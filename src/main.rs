extern crate get_input;
use get_input::get_num_bw;

fn main() {
    let input: u32 = get_num_bw(1 ,u32::MAX,"Order of fibonacci number?");

    let (mut i,mut f,mut n) = (0, 0, 1);

    let output: u32 = loop {
        i += 1;
        print!("{n} ");
    	if i == input {
    		println!();
    		break n;
    	}
    	n = n + f;
    	f = n - f;
    	
    };

    println!("The no. is {}", output);
}
