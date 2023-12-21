fn main() {
    println!("{}", lab_rust_crud::lib());
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(lab_rust_crud::lib(), "Hi!");
    }
}
