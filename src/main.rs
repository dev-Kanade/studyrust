fn main() {
    println!("Hello, world!");
    ifs();
}


fn ifs(){
    println!("これはif文のべんきょーです。");
    const A:&str = "Hello";
    let mut c = 1;
    let mut b = 2;
    if c==2{
        println!("cは2です。{A}");
    }else{
        println!("{A}");
    };
}