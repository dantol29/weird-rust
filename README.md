# weird-rust
Painful transition from lovely C to diabolical Rust

1. ### Ownership model ~~Manual memory managment~~
   _Fighting with the borrow checker_
   
   **Golden Rules**
   1. You cannot borrow a variable as mutable more than once at a time
   2. You cannot have a mutable reference if an immutable reference exists
   3. When the owner goes out of scope, the value is dropped(freed)

   <br>

    **Things that I find confusing:**
     <details>
    <summary><i>1. You want 2 variables to point to the same memory address?</i></summary>

   ```rust
   let s1: String = String::from("hello"); // creating a pointer to the allocated string
   
   let s2: String = s1; // creating a shallow copy of s1 (u might think)

   println!("Haha, {}", s1); // compile time error
   ```

    <ins>Explanation</ins>:
   
   Here `let s2: String = s1;` variable `s1` is moved to `s2`, therefore `s1` is empty and cannot be used anymore. In Rust, when you assign a value to another variable without explicitly copying it, ownership is moved from the original variable to the new variable. In C++ it is done with `std::move()`. **Rust defaults to moving a value**.

   <ins>Solution</ins> - Use references(Borrowing). We are borrowing value, but we do not actually take ownership of it:
   ```rust
   let s1: String = String::from("hello"); // creating a pointer to the allocated string
   
   let s2: String = &s1; // creating a reference (borrowing a variable)

   println!("OK, {}", s1); // yee
   ```
   
    </details>
  
     <br>

     <details>
      <summary><i>2. You wanna pass a variable to a function?</i></summary>

   ```rust
   let s: String = String::from("Hey!"); // creating a pointer to the allocated string
   randomFunction(s); // pass the pointer to the function
   println!("{}", s); // compile time error

   fn randomFunction(word: String) {
      println!("{}", word);
   }
   ```

   <ins>Explanation</ins>:

   Here `randomFunction(s);` passing in `s` moves it to `word` variable, therefore `s` becomes empty and cannot be used anymore. In C the function would expect a pointer to the memory address, but in Rust only 1 varible can point to the memory address of a variable. **Rust defaults to moving a value**

   <ins>Solution</ins> - Use references(Borrowing). We are borrowing value, but we do not actually take ownership of it:
    ```rust
    let s: String = String::from("Hey!"); // creating a pointer to the allocated string
    randomFunction(&s); // pass the pointer to the function
    println!("{}", s); // yee

    fn randomFunction(word: &String) {
       println!("{}", word);
    }
    ```

    </details>

    <br>
    
   <details>
      <summary><i>3. You wanna modify a variable?</i></summary>

   ```rust
   let x: i32 = 5; // creating 32-bit integer

   x = 10; // compile time error
   ```

   <ins>Explanation:</ins>

   In Rust, **variables are immutable by default**. This means that once a value is assigned to a variable, it cannot be changed unless you explicitly specify that the variable is mutable. In contrast, C variables are mutable by default.

   <ins>Solution</ins> - use keyword `mut`. Explicitly specify that the variable is mutable:
   ```rust
   let mut x: i32 = 5; // creating mutable 32-bit integer

   x = 10; // yeee
   ```

</details>
My thoughts:

      The good thing is that Rust does all the checks at compile time, therefore it does not affect code efficiency. 
      Borrowing rules are hard to follow but they prevent data races, memory leaks and a bunch of errors that you have to deal with in C.
