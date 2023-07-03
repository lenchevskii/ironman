fn main() {
    fn match_one(opt: Option<i32>) -> String {
        match opt {
            Some(1) => "Success".to_string(),
            None => "Failure".to_string(),
            _ => "Other option".to_string(),
        }
    }

    println!("{}", match_one(Some(1)));
}
