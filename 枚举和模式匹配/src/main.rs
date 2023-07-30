fn main() {
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // 在这里定义方法体
            dbg!(&self);
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
    //Option
    let some_number: Option<i32>=Some(5);
    match some_number{
        Some(i)=>{println!("有值:{}",i)},
        _=>{println!("没有值")},
    }
    if let Some(i)=some_number{
        println!("有值:{}",i);
    }
}