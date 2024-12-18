use std::io;

fn gcd(a :u64, b :u64) -> u64
{
    if b==0 {return a;}
    return gcd(b, a%b);
}

fn main()
{
    println!("Enter Text: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Error !");
    let intinput1 :u64 = input1.trim().parse().unwrap();

    println!("Enter Text: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Error !");
    let intinput2 :u64 = input2.trim().parse().unwrap();

    println!("GCD: {}", gcd(intinput1,intinput2));

    let res = 33_f64.powf(66_f64);
    let r = res % 333.0;
    println!("Power: {}", res);
    println!("Power: {}", r);
}