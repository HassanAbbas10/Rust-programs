use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the game ✨");

    println!("Press 1 to enter the game \n");

    println!("Press 2 to exit the game \n");

    fn get_input_str() ->String{

        let mut input2 = String::new();
            io::stdin()
            .read_line(&mut input2)
            .expect("String input has not been read");
        return input2;
    }

    

    fn get_input()-> i32{
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");

        let input: i32 = input.trim().parse().expect("Invalid input");
        return input;
    }

    let start_game = get_input() ;

    if start_game == 1 {
        println!("Welcome (●'◡'●) \n");
        println!("==================You have entered the game========================= \n");
        println!(" ===========>++Enter the number++<=========== \n");

        let magic_num = rand::thread_rng().gen_range(1..=100);
        let guess_num = get_input_str();

        if magic_num == guess_num.trim().parse().expect("input error") {
            println!("\n=========You Win============\n Numbers are matched");
        } else {
            println!("\n=========You lose========\n Your number {guess_num} does not match magic number : {magic_num}");
        }

        
    } else if start_game == 2 {
        println!("\nYou have exit the game \n ");
    } else {
        println!("Invalid input");
    }
}
