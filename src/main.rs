use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::io::{self, Write};
use ulid::Ulid;

//The Book Struct
struct Book {
    book_name: String,
    book_id: String,
    book_author: String,
    book_quantity: i32,
}

//The Struct for the schema of a borrowed book. Obvious thing lol ;)
struct BorrowedBook {
    book_id: String,
    user_id: i32,
    borrow_quantity: i32,
    borrow_id: String,
}

//Display trait implementation for the BorrowedBook struct so that it gets printed when iterated
impl Display for BorrowedBook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Book ID {}\nBorrowed by User ID {}\nWith Borrow Quantity {}\nWith Borrow ULID {}",
            self.book_id.to_string(),
            self.user_id.to_string(),
            self.borrow_quantity,
            self.borrow_id
        )
    }
}

fn main() {

    //The borrowed books are recorded in a file. If the file doesn't exist, it gets created.
    let mut borrowed_books_file = OpenOptions::new()
        .write(true)
        .open("src/borrowed_books_file.csv")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("src/borrowed_books_file.csv").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });

    let mut choice = String::new(); //The choice user makes from menu options
    
    let mut borrow_quantity: String = String::new();
    let mut borrow_book_id: String = String::new();

    let mut borrowed_books: Vec<BorrowedBook> = Vec::new();

    //List of available books
    //ToDo: Put this in a CSV file
    let mut books = vec![
        Book {
            book_name: String::from("Book A"),
            book_author: String::from("Author A"),
            book_id: String::from("1"),
            book_quantity: 10,
        },
        Book {
            book_name: String::from("Book B"),
            book_author: String::from("Author B"),
            book_id: String::from("2"),
            book_quantity: 10,
        },
    ];

    //Labelled infinite menu loop
    'menu_loop: loop {
        println!("Welcome to Yet-Another-Library");
        println!("------------------------------\n");
        println!("Enter your Choice: ");
        println!("1. List Available Books");
        println!("2. Borrow a Book");
        println!("3. List Borrowed Books");
        println!("4. Quit");
        println!("Enter Your Choice: ");

        //Since read_line() appends to the given string, previous data needs to be cleaned
        choice.clear();
        
        //Reading the user input. So obvious haha
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        println!("Your Choice: {choice}");

        match choice.as_str().trim() {
            "1" => {
                //ToDo: Implement Display trait for Book
                for i in books.iter() {
                    println!("-----------------------");
                    println!("Name: {}", i.book_name);
                    println!("Author: {}", i.book_author);
                    println!("ID: {}", i.book_id);
                    println!("-----------------------");
                }
            }
            "2" => {
                //Cleaning previous inputs if any. It's obvious now...
                borrow_book_id.clear();
                borrow_quantity.clear();

                println!("Enter Book ID");
                io::stdin()
                    .read_line(&mut borrow_book_id)
                    .expect("Failed to read ID");

                println!("Enter Borrow Quantity");
                io::stdin()
                    .read_line(&mut borrow_quantity)
                    .expect("Failed to read quantity");

                println!(
                    "\nYou Chose:
Quantity = {borrow_quantity}Book ID = {borrow_book_id}"
                );

                //Checking input sanity

                let borrow_book_id = borrow_book_id.trim();

                if borrow_book_id.len() == 0 {
                    println!("Invalid Book ID");
                    continue 'menu_loop;
                }

                //If the borrow quantity cannot be converted to a number means that user entered something else. So it gets set to -1 here
                let borrow_quantity = borrow_quantity.trim().parse::<i32>().unwrap_or(-1);

                if borrow_quantity < 0 {
                    println!("Invalid Borrow Quantity");
                    continue 'menu_loop;
                }

                if borrow_quantity == 0 {
                    println!("Borrow Quantity cannot be 0")
                }

                //Finding the book user wants. Mutable reference is obtained because it's quantity needs to reduced
                let selected_book = books.iter_mut().find(|book| book.book_id == borrow_book_id);

                match selected_book {
                    Some(book) => {

                        if book.book_quantity <= 0 {
                            println!("Book Not Available");
                            return;
                        }

                        //Keeping the borrow id as ULIDs so that they are easier to sort. Ask ChatGPT if you don't know what ULIDs are
                        let borrow_id = Ulid::new().to_string();

                        let borrowed_book_obj = BorrowedBook {
                            book_id: book.book_id.clone(),
                            user_id: 1,
                            borrow_quantity: borrow_quantity,
                            borrow_id: borrow_id.clone(),
                        };

                        //Borrowed books csv string to write to file
                        let csv = format!(
                            "\n{},{},{},{}",
                            borrowed_book_obj.book_id.to_string(),
                            borrowed_book_obj.user_id.to_string(),
                            borrowed_book_obj.borrow_quantity.to_string(),
                            borrowed_book_obj.borrow_id.to_string()
                        );

                        borrowed_books.push(borrowed_book_obj);

                        book.book_quantity -= 1;

                        match borrowed_books_file.write_all(csv.as_bytes()) {
                            Ok(_) => {}
                            Err(_) => {}
                        }

                        println!(
                            "Book Named {}\nWith ID {}\nBorrowed By {}\nWith Borrow Quantity {}\nWith Borrow ULID {}",
                            book.book_name, book.book_id, 1,borrow_quantity, borrow_id
                        )
                    }
                    None => println!("No Book Found"),
                }
            }
            "3" => match borrowed_books.len() {
                0 => println!("No Books Borrowed!!"),
                _ => {
                    for i in borrowed_books.iter() {
                        println!("{}", i)
                    }
                }
            },
            "4" => break,
            _ => println!("Invalid Choice"),
        }
    }
}
