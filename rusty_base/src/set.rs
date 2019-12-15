
use std::collections::HashSet;

use std::hash::{Hash, Hasher};

#[derive(Eq, Debug)]
struct Viking {
    name: String,
    power: usize,
}

impl PartialEq for Viking {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

//hash object
impl Hash for Viking{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}


pub fn studyHashSet(){
    let mut books = HashSet::new();

    // Add some books.
    books.insert("A Dance With Dragons".to_string());
    books.insert("To Kill a Mockingbird".to_string());
    books.insert("The Odyssey".to_string());
    books.insert("The Great Gatsby".to_string());

    books.insert("Java".to_string());
    books.insert("Java".to_string());


    // Check for a specific one.
    if !books.contains("The Winds of Winter") {
        println!("We have {} books, but The Winds of Winter ain't one.",
                books.len());
    }

    // Remove a book.
    books.remove("The Odyssey");

    // Iterate over everything.
    for book in &books {
        println!("{}", book);
    }

    let mut vikings = HashSet::new();

    vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
    vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
    vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });

    let harald1 = Viking { name: "Harald".to_string(), power: 8 };
    let harald2 = Viking { name: "Harald".to_string(), power: 10 };

    println!("==================harald============================");
    assert!(harald1 == harald2);
    println!("harald.eq:{}",harald1==harald2);

    vikings.insert(harald1);

    vikings.insert(harald2);
    
    // Use derived implementation to print the vikings.
    //wu xu 
    for x in &vikings {
        println!("{:?}", x);
    }

    let str1 = String::from("str1");
    let str1_2 = String::from("str1");
    let str2 = String::from("str2");
    let str3 = String::from("str3");
    println!("str1==str2:{}",str1==str1_2);
    // match str1.as_str() {
    //     str2.as_str()=>println!("0")
    // }
    assert_eq!(str1, str1_2);

}