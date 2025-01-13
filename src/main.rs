use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");

    let x = 4;
    let y: u32= 5;

    println!("x is : { }",x);
    println!("y is : { }",y);

    let mut z = 6;
    println!("z is : { }",z);
    z = 10;
    println!("z is : { }",z);

    let a = 7;
    let a = a + 7;
    println!("a is : { }",a);

    //Name Shadowing
    {
        let a = 10;
        println!("a is : { }",a);
    }

    let a = "abc";
    println!("a is : { }",a);

    const SECONDS_IN_A_MINUTE: u32 = 10;
    println!("SECONDS_IN_A_MINUTE is : { }",SECONDS_IN_A_MINUTE);

    println!("{} is even.",is_even(20));

    let v: i32 = 1;
    println!("{}",fib(v));

    let my_string = String::from("Hello, world!");
    let length = get_string_length(&my_string);
    println!("{}",length);


    //struct
    let react = Rect{
        width:20,
        height:70,
    };
    println!("{}",react.area());

    // option enums
    let index = find_first_a(String::from("preet"));

    match index{
        Some(value) => println!("index is {}",value),
        None => println!("a not found"),
    }

    // result enum
    let ans = read_from_file_hello(String::from("a.txt"));

    //vectors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}",vec);

    println!("Print all even : {:?}",even_filter(vec));
}

fn even_filter(vec: Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();
    for val in vec{
        if val % 2 == 0{
            new_vec.push(val);
        }
    }

    return new_vec;
}



fn read_from_file_hello(file_path: String) -> Result<String,String>{
    let result = read_to_string(file_path);
    match result{
        Ok(data) => Ok(data),
        Err(_err) => Err(String::from("File not read")),
    }
}

fn find_first_a(s: String) -> Option<i32>{
    for (index,char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}

struct User{
    active:bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rect{
    width: u32,
    height: u32,
}

impl Rect{
    fn area(&self) -> u32{
        return self.width * self.height
    }
}

fn is_even(num: i32) -> bool {
    if num%2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return 1;
    }

    for _ in 1..num - 2 {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
    
}

fn get_string_length(s: &str) -> usize{
    s.chars().count()
}
