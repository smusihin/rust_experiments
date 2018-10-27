use std::io;

fn main() {       
    println!("Enter a number:");
    let mut number = String::new();

    io::stdin().read_line(&mut number)
	.expect("Line was not readed");

    match number.trim().parse(){
	    Ok(num) => println!("{}nth Fibonachi number is {}",num,fib_nth(num)),
	    Err(_) => {}
	};
	
	
}

fn fib_nth(number: u32)->u32
{
    let mut prev = 0;
    let mut current = 1;
    for i in 0..number
    {
        let next = prev + current;
        prev = current;
        current = next;
        println!("{} - {}",i+1, prev);   
    }
    prev    
}
