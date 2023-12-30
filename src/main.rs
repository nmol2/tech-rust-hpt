fn main() {
    let fullname = String::from("Kannan,Sudhakaran,Tutorialspoint");
    let dt: &str = &fullname;
    let tokens: Vec<&str> = fullname.split(",").collect();
    println!("{} {}", tokens[2], dt);
}
