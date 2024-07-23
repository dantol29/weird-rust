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


### 2. Structs ~~OOP~~
   _Structs are pretty similar to C but `impl` feature makes them more flexible_

   `Impl`(implementation block) is a collection of functions similar to public member functions from C++ OOP world
   ```rust
   struct Rectangle {
      width: u64,
      height: u64
   }

   impl Rectangle {
      fn area(&self) -> u64 {       // first argument is always self which is the instance the method is called on
         self.width * self.height;  // it is also possible to take the ownership of an instance
      }                             // but in this case we only need a reference

      fn can_hold(&self, other: &Rectangle) -> bool {
         self.height > other.height && self.width > other.width;
      }
   }
   ```

   <ins>Associated functions</ins> - are defined in `impl` blocks and do not take a self parameter. They are similar to static member functions in C++ because they can be called on the type itself rather than on an instance of the type.
   ```rust
   struct Rectangle {
      width: u64,
      height: u64
   }

   impl Rectangle {
      fn square(number: i32) -> i32 {      // associated function (does not take self parametr)
         number * number
      }

      fn area(&self) -> u64 {
         self.width * self.height;       // method (takes self parametr)
      }
   }

   fn main() {
      let r: Rectangle = Rectangle { width: 40, height: 50 }; // creating struct variable
      Rectangle::square(2);             // calling associated function
      r.area()                          // calling method
   }
   ```
### 3. Match ~~Switch~~ and Enum
_Match statements must cover all possible values. This ensures that every possible case is handled_

Match works especially good with enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },      // enums in Rust can have options
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_message(msg: &Message) {
    match msg {                  // match statement
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        _ => println!("Message is not supported")                      // _ means anything else
    }
}

fn main() {
   let write_message = Message::Write(String::from("Hello, Rust!"));  // use enum with options

   process_message(&write_message);                                   // pass a reference to avoid passing an ownership
}
```


### 4. Collections ~~Containers~~
1. `Vec` - similar to `std::vector`
   ```rust
   let mut r: Vec<i32> = Vec::new(); // creating a heap-allocated array of integers

   r.push(1);                       // adding elements to the array

   match r.get(2) {                 // safe out-of-bounds access with get() and match
      Some(val) => println!("Print what you want {}", val),
      None => println!("Index is out of bounds!")
   }
   
   ```
   The only thing that feels strange is the way you iterate...
   ```rust
   for x: &i32 in &r {               // iterating over an immutable vector(do not forget to borrow)
      println("Element is {}", x);   // println! automatically dereferences a variable
   }

   for x: &mut i32 in &mut r {       // iterating over a mutable vector
      *x += 20;                      // DEREFERENCE to modify an element 
   }

   ```

   <ins>Explanation</ins>:

   In C, when working with arrays you have to dereference it's elements. Usually it is automatically dereferenced by compiler when `[]` is used. `arr[i]` is equivalent to `*(arr + i)`. Rust compiler also does it. Although, when iterating over an array(`for i in &arr`) the memory adress of the element is returned, therefore to access the value of the address `*` operator is needed. Note that `println!` macro automatically dereferences variables.


## ICP Specific Rust
- `thread_local!` - creates a thread-local variable, and the data associated with this variable is unique to each thread. Used when static variables needed.
```rust
thread_local! {
    static ACTIVE_USERS: RefCell<UserMap> = RefCell::new(UserMap::new());
}
```
- `RefCell<T>` - allows to mutate a value even if the variable itself is immutable. In ICP canister development, many functions are async. These functions might need to modify shared state or perform operations that are not possible under strict compile-time borrowing rules.
```txt
Key Methods

borrow - acquires an immutable borrow. Returns a Ref<T>, which provides access to the inner data.

borrow_mut - acquires a mutable borrow. Returns a RefMut<T>, which provides mutable access to the inner data.

replace - replaces the inner value and returns the old value.
```
- `static` variables - global variables that the protocol preserves across upgrades. For example, a user database should probably be static.
```rust
static USERS: RefCell<Users> = RefCall::new(Users::new();
```
- `.with()` - used to access and modify the thread-local data. Ensures that the closure you provide has access to the thread-local data for the duration of the closure’s executio
```rust
COUNTER.with(|counter| *counter.borrow_mut() += 1);
```
