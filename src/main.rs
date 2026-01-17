fn main() {
    println!("Hello, world!");
    ifs();
}


fn ifs(){
    println!("これはif文のべんきょーです。");
    const A:&str = "Hello";
    let c = 1;
    let mut _b = 2;
    if c==2{
        println!("cは2です。{A}");
    }else{
        println!("{A}");
    };
}

fn _loops() {
    let mut count = 0;

    loop {
        println!("count = {}", count);
        count += 1;

        if count == 50000 {
            break;           // ループ脱出
        }
    }

    println!("終了！");
}