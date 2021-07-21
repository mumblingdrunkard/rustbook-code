struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// literally how drop is implemented
fn my_drop<T>(_x: T) {}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointers created");
    my_drop(c); // Takes ownership and drops at the end of the function
    println!("CustomSmartPointer dropped before the end of main.");
}
