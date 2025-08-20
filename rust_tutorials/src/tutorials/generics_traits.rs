pub fn generics_demo() {
    println!("generics_demo begins here");

    // max of i32 vector
    let numbers1: Vec<i32> = vec![1,2,3,6,4,5,5];
    let max = *find_largest(&numbers1);
    println!("max1: {}", max);

    // max of u8 vector
    let numbers2: Vec<u8> = vec![1,2,6,4,10,5];
    let max = *find_largest(&numbers2);
    println!("max2: {}", max);

    // make multi-generic type struct instances
    let pair1 = Pair {_k: 'a', _v: 3.0};
    let pair2 = Pair {_k: 2, _v: 'b'};
    println!("struct pair1 = {:?}", pair1);
    println!("struct pair2 = {:?}", pair2);

    // make multi-generic type enum instance
    let id1: MyIdentity<i32, String> = MyIdentity::ID1(49485);
    let id2: MyIdentity<i32, String> = MyIdentity::ID2(String::from("a.hack@hamil.com"));
    println!("enum id1 = {:?}", id1);
    println!("enum id2 = {:?}", id2);

}

fn find_largest<T: PartialOrd>(vec_ref: &Vec<T>) -> &T {
    let mut max = &vec_ref[0];
    for elem in vec_ref {
        // the > operator can only be used for types implementing the PartialOrd trait
        if *elem > *max {
            max = elem;
        }
    }
    max
}

#[derive(Debug)]
struct Pair<T,U> {
    _k: T,
    _v: U
}

#[derive(Debug)]
enum MyIdentity<T, E> {
    ID1(T),
    ID2(E)
}
