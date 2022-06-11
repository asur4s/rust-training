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
    let list = Cons(
        1,
        Box::new(
            Cons(2, Box::new(Nil))
        )
    );

}


pub fn test_deref(){
    let x= 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}