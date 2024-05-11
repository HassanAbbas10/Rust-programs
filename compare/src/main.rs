use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    fn get_inp_int() -> i32 {
        //make empty string
        let mut _input = String::new();
        //take input in that string
        io::stdin()
            .read_line(&mut _input)
            .expect("Input is not taken from user");

        //changing input from string to integer
        let _input = _input
            .trim()
            .parse()
            .expect("integer conversion in fn_inp has gone wrong");
        return _input;
    }
        //taking randm num
    let _magicnum = rand::thread_rng().gen_range(1..=25);
    println!("Please enter your number");
    let _usernum = get_inp_int();
    println!("This is your guessed number : \n {_usernum} \n");
    println!("This is the random number : \n {_magicnum}\n");

  match _magicnum.cmp(&_usernum){
    Ordering::Less => println!("this is the low"),
    Ordering::Equal => println!("this is the Equal"),
    Ordering::Greater => println!("this is the Greater")
  }


}
