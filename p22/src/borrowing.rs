/// # Rust Borrowing Examples
///
/// This module demonstrates various borrowing concepts in Rust with practical examples.
/// It covers:
/// - Ownership
/// - Immutable borrows
/// - Mutable borrows
/// - Borrowing rules
/// - Lifetimes
/// - Advanced borrowing patterns

/// ## Basic Ownership Example
///
/// This function demonstrates how ownership works in Rust.
/// ```
/// use p22::borrowing::ownership_example;
/// ownership_example();
/// ```
pub fn ownership_example() {
    // When you assign a value to a variable, that variable takes ownership
    let s1 = String::from("hello");
    
    // When you assign it to another variable, ownership moves to the new variable
    let s2 = s1;
    
    // s1 is no longer valid to use, it's been moved
    // Uncommenting the next line would cause a compile error:
    // println!("s1: {}", s1);  // ERROR: value borrowed here after move
    
    println!("s2: {}", s2);  // This works fine
    
    // Passing to a function also transfers ownership
    takes_ownership(s2);
    
    // Can't use s2 anymore, ownership was moved to the function
    // Uncommenting this would cause an error:
    // println!("s2 after: {}", s2);  // ERROR: value borrowed here after move
    
    // For primitive types that implement Copy trait, a copy is made instead of a move
    let x = 5;
    let y = x;  // x is copied, not moved
    
    println!("x: {}, y: {}", x, y);  // Both are valid
}

/// Helper function that takes ownership of a String
fn takes_ownership(s: String) {
    println!("Function took ownership of: {}", s);
    // s is dropped when this function ends
}

/// ## Immutable Borrowing Example
///
/// This function demonstrates immutable borrowing (shared references).
/// ```
/// use p22::borrowing::immutable_borrow_example;
/// immutable_borrow_example();
/// ```
pub fn immutable_borrow_example() {
    let s = String::from("hello");
    
    // Pass a reference to the function instead of transferring ownership
    let length = calculate_length(&s);
    
    // We can still use s because we only lent it to the function
    println!("The length of '{}' is {}.", s, length);
    
    // We can have multiple immutable references at the same time
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
}

/// Function that borrows a string immutably and returns its length
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
    // s goes out of scope here, but since it doesn't have ownership,
    // nothing happens to the value it's referring to
}

/// ## Mutable Borrowing Example
///
/// This function demonstrates mutable borrowing (exclusive references).
/// ```
/// use p22::borrowing::mutable_borrow_example;
/// mutable_borrow_example();
/// ```
pub fn mutable_borrow_example() {
    let mut s = String::from("hello");
    
    // Pass a mutable reference to the function
    append_world(&mut s);
    
    // We can still use s, and it's been modified
    println!("Modified string: {}", s);
    
    // Mutable borrow rules: only one mutable reference at a time
    let r1 = &mut s;
    r1.push_str(" again");
    
    // Uncommenting this would cause an error:
    // let r2 = &mut s;  // ERROR: cannot borrow `s` as mutable more than once at a time
    // println!("r1: {}, r2: {}", r1, r2);
    
    println!("After another modification: {}", r1);
    
    // Once r1 is no longer used, we can make another mutable borrow
    let r2 = &mut s;
    r2.push('!');
    
    println!("Final string: {}", r2);
}

/// Function that borrows a string mutably and modifies it
fn append_world(s: &mut String) {
    s.push_str(" world");
}

/// ## Borrowing Rules Example
///
/// This function demonstrates the borrowing rules.
/// ```
/// use p22::borrowing::borrowing_rules_example;
/// borrowing_rules_example();
/// ```
pub fn borrowing_rules_example() {
    let mut s = String::from("hello");
    
    // Rule 1: You can have either:
    //   * One mutable reference
    //   * Any number of immutable references
    
    // This is fine - multiple immutable references
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    
    // This is also fine - immutable references are no longer used after this point
    // so we can create a mutable reference
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3: {}", r3);
    
    // Uncommenting these would cause an error:
    // let r4 = &s;  // ERROR: cannot borrow `s` as immutable because it is also borrowed as mutable
    // println!("r3: {}, r4: {}", r3, r4);
    
    // Rule 2: References must always be valid
    // No dangling references are allowed in Rust
}

/// ## Lifetime Example
///
/// This function demonstrates basic lifetime concepts.
/// ```
/// use p22::borrowing::lifetime_example;
/// lifetime_example();
/// ```
pub fn lifetime_example() {
    let string1 = String::from("long string is long");
    
    {
        let string2 = String::from("short");
        
        // Result will point to whichever string is longer
        let result = longest(&string1, &string2);
        println!("The longest string is: {}", result);
    }
    
    // string2 is dropped here, so we can't return a reference to it from longest
    // This is safe because result now points to string1, which is still valid
    
    // This would fail compilation if longest could return a reference to string2,
    // but the lifetime system prevents this error
}

/// Function with explicit lifetime annotations
/// The 'a lifetime parameter indicates that the return value will live at least
/// as long as the shorter of the two input references
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// ## Structs with References Example
///
/// This function demonstrates using references in structs.
/// ```
/// use p22::borrowing::structs_with_references_example;
/// structs_with_references_example();
/// ```
pub fn structs_with_references_example() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    
    // This instance of Excerpt can't outlive the String it references
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", excerpt.part);
}

/// A struct that holds a reference, requiring a lifetime annotation
pub struct Excerpt<'a> {
    part: &'a str,
}

/// ## Ref Pattern Example
///
/// This function demonstrates the ref pattern in pattern matching.
/// ```
/// use p22::borrowing::ref_pattern_example;
/// ref_pattern_example();
/// ```
pub fn ref_pattern_example() {
    let mut value = 5;
    
    // Using `ref` to create a reference during pattern matching
    match value {
        ref r => println!("Got a reference to {}", r),
    }
    
    // Using `ref mut` to create a mutable reference
    match value {
        ref mut r => *r += 1,
    }
    
    println!("Value is now: {}", value);
    
    // Another example with a tuple
    let tuple = (String::from("hello"), 5);
    
    // Destructuring while creating references
    let (ref s, ref n) = tuple;
    
    println!("String reference: {}, number reference: {}", s, n);
    
    // Can't use tuple here if we moved out of it, but we only borrowed with `ref`
    println!("Original tuple: ({}, {})", tuple.0, tuple.1);
}

/// ## Returning References Example
///
/// This function demonstrates when and how to return references.
/// ```
/// use p22::borrowing::returning_references_example;
/// returning_references_example();
/// ```
pub fn returning_references_example() {
    let sentence = String::from("The quick brown fox jumps over the lazy dog");
    
    // Get a reference to a word in the sentence
    let word = first_word(&sentence);
    println!("First word: {}", word);
    
    // This works because 'word' is a reference into 'sentence'
    // and sentence is still valid here
    
    // We couldn't modify sentence while word is being used:
    // sentence.clear();  // ERROR: cannot borrow `sentence` as mutable because it is borrowed as immutable
    // println!("First word after clear: {}", word);
}

/// Returns a reference to the first word in a string
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/// ## Slices Example
///
/// This function demonstrates string and other slices.
/// ```
/// use p22::borrowing::slices_example;
/// slices_example();
/// ```
pub fn slices_example() {
    let s = String::from("hello world");
    
    // String slices are references to parts of a String
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Slices: '{}' and '{}'", hello, world);
    
    // Using arrays and slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    
    println!("Array slice: {:?}", slice);
}

/// ## Interior Mutability Example
///
/// This function demonstrates the RefCell type for interior mutability.
/// ```
/// use p22::borrowing::interior_mutability_example;
/// interior_mutability_example();
/// ```
pub fn interior_mutability_example() {
    use std::cell::RefCell;
    
    // RefCell allows mutation through a shared reference (interior mutability)
    let data = RefCell::new(5);
    
    // Create an immutable reference to RefCell
    let reference = &data;
    
    // But we can still mutate the value inside it
    {
        // Borrow mutably - this happens at runtime
        let mut value = reference.borrow_mut();
        *value += 1;
    } // mutable borrow ends here
    
    // Now we can borrow immutably
    {
        let value = reference.borrow();
        println!("Value after mutation: {}", *value);
    }
    
    // RefCell enforces borrowing rules at runtime:
    // - Multiple immutable borrows: OK
    // - One mutable borrow: OK
    // - Mix of mutable and immutable borrows: panic at runtime
}

/// ## Shared Ownership Example
///
/// This function demonstrates Rc (reference counting) for shared ownership.
/// ```
/// use p22::borrowing::shared_ownership_example;
/// shared_ownership_example();
/// ```
pub fn shared_ownership_example() {
    use std::rc::Rc;
    
    // Rc allows multiple owners of the same data
    let data = Rc::new(String::from("shared data"));
    
    // Create multiple owners by cloning the Rc
    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data);
    
    // All owners point to the same data
    println!("Owner1: {}", owner1);
    println!("Owner2: {}", owner2);
    println!("Original: {}", data);
    
    // Count the number of owners
    println!("Reference count: {}", Rc::strong_count(&data));
    
    // Note: Rc only allows immutable access - if you need mutation,
    // combine with RefCell or use other types like Mutex
}

/// ## Self-referential Structs Example
///
/// This function demonstrates a pattern for creating self-referential structs.
/// ```
/// use p22::borrowing::self_referential_example;
/// self_referential_example();
/// ```
pub fn self_referential_example() {
    // Create the data and reference together to ensure lifetimes work out
    let mut data_holder = SelfReferential::new("hello world");
    
    println!("Data: {}, First word: {}", 
        data_holder.data(), 
        data_holder.first_word_ref());
    
    // We can still modify the data through provided methods
    data_holder.append_data(" and more");
    
    println!("Updated data: {}, First word still: {}", 
        data_holder.data(), 
        data_holder.first_word_ref());
}

/// A struct that holds both data and a reference to part of that data
/// (requires careful handling of lifetimes)
pub struct SelfReferential<'a> {
    data: String,
    // A reference to the first word in data
    first_word: &'a str,
}

impl<'a> SelfReferential<'a> {
    /// Create a new instance with computed references
    pub fn new(s: &str) -> Self {
        let data = String::from(s);
        // Temporarily create a reference for computing first_word
        let first_word = Self::get_first_word(&data);
        
        // Using unsafe is one way to handle self-referential structures
        // Another approach is to use crates specifically designed for this purpose
        // like owning_ref or ouroboros
        
        // Safety: We ensure that first_word is valid for the lifetime of data
        // by keeping both in the same struct and not allowing direct mutation of data
        let first_word = unsafe { std::mem::transmute::<&str, &'a str>(first_word) };
        
        Self { data, first_word }
    }
    
    /// Helper function to get the first word
    fn get_first_word(s: &str) -> &str {
        match s.find(' ') {
            Some(pos) => &s[0..pos],
            None => s,
        }
    }
    
    /// Get a reference to the data
    pub fn data(&self) -> &str {
        &self.data
    }
    
    /// Get the first word reference
    pub fn first_word_ref(&self) -> &str {
        self.first_word
    }
    
    /// Append to the data (note: this doesn't update first_word)
    pub fn append_data(&mut self, suffix: &str) {
        self.data.push_str(suffix);
        // Note: first_word stays valid because we don't modify the beginning of the string
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_length() {
        let s = String::from("hello");
        assert_eq!(calculate_length(&s), 5);
    }
    
    #[test]
    fn test_append_world() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello world");
    }
    
    #[test]
    fn test_longest() {
        let s1 = String::from("long");
        let s2 = String::from("longer");
        assert_eq!(longest(&s1, &s2), "longer");
    }
    
    #[test]
    fn test_first_word() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
    }
}