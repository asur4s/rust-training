pub fn hello() {
    println!("{:?}", "Hello");
}

pub fn test_box() {
    let b = Box::new(5);
    println!("b = {:?}", b);
}

pub fn test_box_list() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    };

    use List::*;
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}

pub fn test_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    fn function_deref(name: &str) {
        println!("[*] function_deref: Hello, {:?}", name);
    }
    let m = MyBox::new(String::from("Rust"));
    // &MyBox(String) -> &String -> &str：函数参数会自动解引用。
    // & 表示借用。
    function_deref(&m);
    // 手动解引用
    function_deref(&(*m)[..])

}
