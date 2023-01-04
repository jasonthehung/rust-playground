struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Jason"),
    };
    drop(c);
    println!("Bye c");

    let d = CustomSmartPointer {
        data: String::from("Other Jason"),
    };

    println!("CustomSmartPointers created.")
}
