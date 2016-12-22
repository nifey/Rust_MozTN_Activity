fn main() {
    println!("Enter the operation to perform ");
    loop {
        println!(" 1.Addition\n 2.Subtraction\n 3.Multiplication\n 4.Division\n 5.Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("can't read from input");
        let choice: u8 = choice.trim().parse().unwrap();
        match choice {
            1...4 => {}
            5 => break,
            _ => {
                println!("Enter a valid choice");
                continue;
            }
        }
        println!("Enter the first number");
        let mut num1 = String::new();
        std::io::stdin().read_line(&mut num1).expect("can't read from input");
        let num1: i32 = num1.trim().parse().unwrap();
        println!("Enter the second number");
        let mut num2 = String::new();
        std::io::stdin().read_line(&mut num2).expect("can't read from input");
        let num2: i32 = num2.trim().parse().unwrap();
        match choice {
            1 => println!("{} + {} = {}", num1, num2, num1 + num2),
            2 => println!("{} - {} = {}", num1, num2, num1 - num2),
            3 => println!("{} * {} = {}", num1, num2, num1 * num2),
            4 => println!("{} / {} = {}", num1, num2, num1 / num2),
            _ => println!("Enter a valid choice"),
        }
    }
}
