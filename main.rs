//QUESTION 1

/*mod sports{
    pub mod cricket
    {
        pub fn print_message()
        {
            println!("You are playing Cricket");
        }
    }
}
use sports::cricket::print_message;
fn main() {
    print_message();
}*/

//QUESTION 2

/*mod lib;
fn main(){
    lib::square::find_square::print_square(2);
}*/

//QUESTION 3

use cube;
fn main(){
    println!("The cube of 3 is {}",cube::cube::find_cube::return_cube(3));
}
