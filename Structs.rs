// Structs
// unit struct:  0 byte
struct FileDirectory;

// tuple struct
struct Color(u8, u8, u8);

// named struct
struct Country {
    popilation: u32,
    capital: String,
}

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}

fn main() {
    let x = FileDirectory;
    takes_file_directory(x);

    let my_color = Color(20, 50, 100);
    println!("The second color is {}", my_color.1);
    println!("The second color is {:?}", my_color.1);

    let korea = Country {
        popilation: 48_000_000,
        capital: "Seoul".to_string(),
    };
    println!("The capital is :{}", korea.capital);
    println!("Country is {} bytes in size", size_of_val(&korea));
}
