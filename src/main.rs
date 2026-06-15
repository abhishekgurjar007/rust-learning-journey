fn main(){
    //Strings
    let mut name = String::from("Abhishek");
    name.push_str(" Gurjar");

    println!("My name is {}", name);

    //&str(String slice)

    let name2 : String = String::from("Patrick Jane");
    let name2_slice : &str = &name2;

    println!("My name is {}", name2_slice);

    //or we can print only a part of the string
    let name3_slice : &str = &name2[0..6];
    println!("Hii there, {}", name3_slice);


}