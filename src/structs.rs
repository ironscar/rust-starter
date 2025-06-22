// basic struct with Debug trait
#[derive(Debug)]
struct User {
    name: String,
    age: u16,
    email: String
}
impl User {
    fn build_user(name: String, age: u16, email: String) -> Self {
        // use shorthand setting of keys with same name
        User{name, age, email}
    }
    fn increment_age(&mut self, incr: u16) {
        self.age += incr;
    }
}

// tuple-like structs
struct Color(i32,i32,i32);
struct Vector(i32,i32,i32);

pub fn structs_demo() {
    println!("Structs demo begins here...");

    // instantiating structs using struct constructor
    let mut user = User::build_user(
        String::from("John"), 20, String::from("john@example.com")
    );
    println!("Basic user = {:?}", user);

    // update values of struct keys if variable is mutable using . operator
    user.age = 40;
    println!("After age update, user = {:?}", user);

    // copy user by reference
    let copy_user1 = build_copy_by_ref(String::from("john2@test.com"), &user);
    println!("Copy user with new email = {:?}", copy_user1);

    // copy user
    let mut copy_user2 = build_copy_user(String::from("john3@test.com"), user);
    println!("Copy user with new email = {:?}", copy_user2);

    // update age by method
    copy_user2.increment_age(15);
    println!("Copy user with incremented age = {:?}", copy_user2);

    // tuple-like structs and destructuring them
    let black = Color(3,1,2);
    let vector = Vector(4,5,8);
    let Vector(x,y,z) = vector;
    println!("black = ({},{},{}), vector = ({},{},{})",
         black.0, black.1, black.2,
         x, y, z
    );
}

fn build_copy_by_ref(email: String, user_ref: &User) -> User {
    // copying by reference fails since name already belongs to user_ref unless we use clone() on name
    User { email, name: user_ref.name.clone(), age: user_ref.age }
}

fn build_copy_user(email: String, user: User) -> User {
    // copy properties from other structs easily while overriding required ones
    User { email, ..user }
}
