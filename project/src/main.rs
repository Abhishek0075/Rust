fn main() {
    println!("Hello world!!");
    let x=3;//Dynamic ie we can change the value of x 
    let y=5;//Static ie we cant change the value of y 
    let sum=x+y;
    let new_float:f64=2.10;
    let new_string:String=String::from("Hello world!!");
    let _new_string2:String=("Forca Barca").to_string();
    println!("The sum is {}",sum);
    println!("Sum = {}",sum);//{} is used to print the sum at the position at {}
    print!("{}{}{}",new_float,new_string,_new_string2);
}   