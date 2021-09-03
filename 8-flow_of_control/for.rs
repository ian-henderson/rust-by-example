// for loops

// for and range

// The for in construct can be used to iterate through an Iterator. One of 
// the easiest ways to create an iterator is to use the range notation a..b. 
// This yields values from a (inclusive) to b (exclusive) in steps of one.

// Let's write FizzBuzz using for instead of while.
#[allow(dead_code)]
fn fizzbuzz() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // Alternatively, a..=b can be used for a range that is inclusive on both 
    // ends, e.g. for n in 1..=100 { ... }
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    println!();
}

// for and iterators

// The for in construct is able to interact with an Iterator in several ways. 
// As discussed in the section on the Iterator trait, by default the for loop 
// will apply the into_iter function to the collection. However, this is not 
// the only means of converting collections into iterators.

// into_iter, iter and iter_mut all handle the conversion of a collection 
// into an iterator in different ways, by providing different views on the 
// data within.

// iter - This borrows each element of the collection through each iteration. 
// Thus leaving the collection untouched and available for reuse after the 
// loop.
fn iter_example() {
    println!("iter_exampmle");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}\n", names);
}

// into_iter - This consumes the collection so that on each iteration the 
// exact data is provided. Once the collection has been consumed it is no 
// longer available for reuse as it has been 'moved' within the loop.
fn into_iter_exampmle() {
    println!("into_iter_exampmle");

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    println!();
    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}

// iter_mut - This mutably borrows each element of the collection, allowing 
// for the collection to be modified in place.
fn iter_mut_example() {
    println!("iter_mut_exampmle");

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}\n", names);
}

fn main() {
    fizzbuzz();
    iter_example();
    into_iter_exampmle();
    iter_mut_example();
}
