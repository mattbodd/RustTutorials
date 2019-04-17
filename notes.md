## Chapter 3
#### Raw identifiers
A raw identifier is a way to reuse a restricted word as an identifier.
`let r#fn` will create a variable named `fn` even though it is normally a reserved word.

#### Variables and Mutability
By default, variables are immutable.  To make a variable mutable, use the prefix `mut` (eg: `let mut x = 5`).  A mutable variable allows the user to change the value that a variable binds to.  A similar concept to immutable variables in imperative programming languages is constants.  There are a few differences between constants and variables.
* You cannot use `mut` with a constant
* A constant in Rust is defined with `const`
* A constant in Rust *must* be type annotated
    * eg: `const MAX_POINTS: u32 = 100_000;`

Shadowing is another way in which a variable's value can change but this time without being marked `mut`.  Any variable, once initialized, can be reinitialized using the `let x = ...` syntax.  Without the `let` keyword, an error is reported by the compiler.  Shadow is helpful in that it can also be used to change the type of a variable in addition to its value.

#### Data Types
Rust is a statically typed language (like Haskell) although there are times when it needs more information to decide what type a value is; for this there are type annotations.

##### Scalar Types
A *scalar* type represents a single value.  Rust has four primary scalar types:
* integers, floating-point numbers, booleans and characters

An integer is a number without a fractional component.  The integer types in Rust are described in the following table.

| Length | Signed | Unsigned |
|:------:|:------:|:--------:|
|  8-bit | `i8`   |  `u8`    |
| 16-bit | `i16`  |  `u16`   |
| 32-bit | `i32`  |  `u32`   |
| 64-bit | `i64`  |  `u64`   |
|128-bit | `i128` |  `u128`  |
|  arch  | `isize`|  `usize` |

Signed numbers are stored using two's complement representation.  Each signed variant can store numbers from **-(2^(n-1))** to **2^(n-1)**  where *n* is the number of bits that a variant uses.  In Rust, all number literals except the byte literal allow a visual seperator such as `1_000`
Rust is able to handle overflow in production and will perform two's complement wrapping to mod the number by its corresponding intended size.  In development, this wrapping will not occur and signal an error instead.

Rust also has two primatives for floating points as well.  Rust supports `f32` and `f64` where the default is `f64` as on most CPU's there is not much difference speed wise between the two types of floats.

The character primative in Rust also supports more than just ASCII.  You can represent accented letters, Chinese, Japanese and Korean characters, emojis and zero-width spaces.

##### Compound Types
*Compound Types* can group multiple values into one type.  Rust has two primative compount types: **tuples** and **arrays**.

A tuple is a general way to group together values of various types into one compound type.  Tuples, like in other languages, are fixed in length at declaration.
Tuples can be accessed with pattern matching:
```
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```
Another way to access the values in a tuple is with 'dot' syntax.  Dot syntax works using the index of an item:
```
let tup = (500, 6.4, 1);
println!("The value of the 2nd element is: {}", tup.1);
```

The array Type is another way to create a collection of multiple values.  Unlike a tuple however, every element in an array must be of the same type.  Unlike most other languages too, arrays must also be fixed in size like tuples.  Arrays are useful when you want your data to be allocated on the stack vs the heap.

To initialize an array, a type and number of elements must be provided:
```
let a: [i32; 5] = [1, 2, 3, 4, 5];
```
Array elements can be accessed using indexing.  For example:
```
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```
Trying to index into a position that is greater than the length of an array in Rust will cause a runtime error - panic.

#### Functions
Rust code uses snake case as the convention for variable names.  To define a function in Rust use the `fn` keyword followed by the function name, parentheticals containing 0 or more parameters, and then curly brackets containing the program body.  For example:
```
fn some_function() {
    ...
}
```

Rust does not require forward declarations meaning you can define functions in any order that you want.  One unique feature of a Rust program definition is that in a function signature, you *must* declare the type of each parameter.

Function bodies are made up of a series of statements optionally ending in an expression.  Rust is an expression based language so it is important to know the difference between a statement and an expression.  A *statement* is an instruction that performs some action and **does not** return a value.  An *expression* evaluates to a resulting value.  Expressions do not include ending semicolons.  If you add a semicolon to the end of an expression, you turn it into a statement which **will not** return a value.

##### Functions with Return Values
Functions can return values to the code that calls them.  We don't name return values but their type is declared after an `->`.  As is the case in many functional programming languages, the return value is in reference to the last value returned by the function.  It is possible to return early from a function using the `return` keyword, but often times the implicit return of the last value is used.
```
fn five() -> i32 {
    5
}
fn main() {
    let x = five();
    println!("The value of x is: {}", x);
}
```

#### Control Flow
Rust does not use parentheticals to define conditions in a control flow.  Rust must always find a boolean in a control condition and will not try to convert non-boolean types to booleans.  `if` statements can also be used inside of `let` statements.  For example:
```
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
}
```

##### Loops
Rust has 3 kinds of loops: `loop`, `while` and `for`.

The `loop` keyword signals to Rust that the body of the loop should run indefinitely - until the programmer tells it explicitly when to stop.  To break out of a `loop`, Rust provides the `break` statement.

The `while` keyword defines a single clear condition that should signal to the loop to keep running until that condition is no longer true.

`for` naturally works with iterators:
```
for element in list.iter() {
    println!("The value is: {}", element);
}
```
`for` is also able to repeat a predetermined number of times using the `Range` syntax.  `Range` looks similar to lazy evaluation in Haskell:
```
for number in (1..4).rev() {
    println!("{}!", number);
}
println!("Liftoff!");
```

### Understanding Ownership
#### Ownership
Ownership is something unique to Rust that makes memory safety guarantees without needing a garbage collector.

Many programming languages implement some form of garbage collection or manual allocation and deallocation of memory.  Rust introduces a third option which uses a system of ownerships with a set of rules that the compiler checks at compile time.  *Ownership features do not slow down your program while its running*.

##### The Stack and Heap
Both the stack and the heap are parts of memory that are available to your code at runtime but they are structured in different ways.  The stack stores values in the order it gets them and removes values in the opposite order, LIFO.  The stack is fast to access and we always know where to put new data - the top.  All data on the stack must also take up a known, fixed size.

Data with a size unknown at compile time or a size that may change can be stored in the heap rather than the stack.  The heap is less organized compared to the stack in that when you want to allocate space on the heap, you need to search for free space that fits your demands.  The OS is responsible for finding this space on the heap and returning a pointer which is the address of that location.  Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so that you don't run out of space are all problems that ownerships addresses.

##### Ownership Rules
* Each value in Rust has a variable that is called its *owner*
* There can only be one owner at a time
* When the owner goes out of scope, the value is dropped

##### Variable Scope
*Scope* is the range within a program from which an item is valid.  When a variable goes out of scope it can be thought of as no longer usable.

##### Memory and Allocation
In the case of a string literal, the contents are known at compile time only because the size and contents will never change.  The `String` type in Rust provides a way to support a mutable, growable piece of text.  To accommodate this, the memory must be requested from the OS at runtime and there must be a way to return this memory to the OS when we finish with the `String`.  The request for memory at runtime is handled by the `String::from` method.  The 'freeing' of memory is a result of Rust's ownership properties.  Memory is automatically returned once the variable that owns it goes out of scope.  When a variable goes out of scope, Rust calls a special function for us.  The function is called `drop` and it is where the author of `String` can put the code to return the memory.  Rust automatically calls drop at the end of closing brackets.

Let's look at the way `String` works in depth.  A `String` is made up of three parts: a pointer to memory that holds the contents of the string, a length and a capacity.  This data is stored on the stack while the contents are stored on the heap.

![Rust figure 4.1](/Users/mbodd/Desktop/RustProjects/RustTuts/Rust4-1.png)

The length is how much memory, in bytes, the contents of `String` is currently using.  The capacity is the total amount of memory, in bytes that the `String` has received from the operating system.  When we make a copy of a `String`, we copy the data (*not* the contents) so that we have a new object on the stack storing address, length and capacity.  Both objects point to the same contents.

To avoid the issue of *double freeing*, rust invalidates the original when a copy is made so that when the original goes out scope, it is not `drop`ed.  What Rust does is essentially make a shallow copy although the invalidation of the original item lends to the operation being called a *move*.  One important point to make here is that Rust will never automatically create a deep copy.  To explicitly create a deep copy, use the `.clone()` function provided by Rust; keep in mind that this is a more expensive operation.

Ownership can be transferred by passing a value as a parameter to a function.  When a value is passed as a parameter to a function, any attempt to access that value will result in a compile-time error.  Returning values can also transfer ownership.

##### References and Borrowing
What can we do if we do not want transfer ownership when passing a value as a parameter.  One way to do it is to pass back the parameter in the function return using a tuple.  This method is unnecessarily complex however.  A more effective way would be to pass a reference.  A reference can be defined using the `&` syntax.
```
fn main() {
    let s1 = String::from("hello");
    let len = calculate_lengt(&s1);
    println!("The length of {} is {}", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len();
}
```
You are not allowed to modify a reference by default unless it is mutable.

##### Mutable References
To allow for a reference to be mutable, simply use the `mut` keyword.
```
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```
It is important to note here that there can only be one mutable reference to a particular piece of data at a time.  For example, the following is not allowed:
```
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;
```
This restriction prevents data races at compile time where a data race is defined as:
* Two or more pointers accessing the same data at the same time
* At least one of the pointers is being used to write the data
* There no mechanism being used to synchronize access to the data

Curly braces can be used to create a new scope, allowing for mutable references that are not *simultaneous*.

Another feature of borrowing is that you cannot borrow as both mutable and immutable.

Rust will also prevent dangling references at compile time by stopping users from returning a reference instead of transferring ownership.

##### The Slice Type
String slices solve an issue in Rust where you may want to operate on a string but run the risk of losing having the original `String` value.  To get a string slide, use the `[0..4]` syntax.  For example:
```
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```
Typically a slice does not use an inclusive end but this can be achieved using `=`:
```
let s = String::from("hello world");
let hello = &s[0..=4];
let world = &s[6..=10];
```
A slice actually works by storing a pointer to bytes within a `String`.  If you want to reference starting at the 0th byte of a string, the initial index can be omitted: `let slice = &s[..2];`.  To reference the last byte of a `String`, the final index can be omitted: `let slice = &s[0..];`

A string literal is actually a string slice.  More specifically, it is a slice pointing to a specific part of a binary.  This is why string literals are immutable - `&str` is an immutable reference.

An efficient implementation of a `first_word` function:
```
fn main() {
    let my_string_literal = "hello world";
    let word = first_word(my_string_literal);
}
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
```
Slices are not limited to strings.  A more general slice type can be used on arrays for example.
```
let a = [1,2,3,4,5];
let slice = &a[1..3];
```

### Chapter 5 - Using Structs to Structure Related Data
A *struct* or *structure* is a data type that lets you name and package together multiple related values that make up a meaningful group.
#### Defining and Instantiating Stucts
Structs are similar to tuples in that they can contain different types.  They are different from a tuple in that each item in a struct must be named.  This allows for items in a struct to be referred to by name.

To define a struct, use the keyword `struct`:
```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
To use a struct after we have defined it, we create an instance of it by specifying values for each of its fields.  For example:
```
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```
To get a specific field from a `struct`, use the `.` notation.  A struct instance can be initialized using a special field init shorthand which allows for the field values to be replaced with values of the same name:
```
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
Rust makes it easy to create a new instance of a struct using an old instance.  To do this, simply update particular fields in a new instance and then use `..old_instance` syntax to have the remaining unspecified fields given the same values as the `old_instance`.  For example:
```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```
Another type of struct is the tuple struct which essentially allows for the naming of a tuple.
```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0,0,0);
let origin = Point(0,0,0);
```
For an example program using structs refer to **./struct_example**.  Notice that the reference to the Rectangle struct is useful for a continuation of the current program if we wish to use rect1 again.

By default, we are unable to print the contents of a struct with a simple action.  Built in data types come with a `Display` that specifies how values should be printed.  To get around printing the contents of a string, replace the standard `{}` format with a `{:?}`.  Additionally annotate the program to allow for debugging using `#derive(Debug)]`

#### Method Syntax
Methods are similar to functions.  They are declared with the `fn` keyword followed by a method name.  They can have parameters as well as a return value.  The difference between a method and a function is that a method is defined within the context of a struct.  The first parameter of a method is always `self` which refers to the instance of the struct the method is being called on.  For an example of method usage, refer to the **./struct_example**.

To define a method within a struct, use an implementation block which is denoted with the `impl` keyword.  Methods can take ownership of `self`, borrow `self` immutably, or borrow `self` mutably.  One might borrow immutably using the `&self` syntax if they only want to read data and do not need to take ownership of it.  If one wanted to change the instance that the method is being called on as part of what the method does, a `&mut self` would be passed as the first parameter.  Methods that take just the `self` parameter are rare in Rust.  Usually the `self` syntax is used when the method transforms `self` into something else and one wants to prevent the caller from using the original instance after the transformation.

In addition to a method, an *associated function* can be defined within an `impl` block.  An associated function is one which is not passed a reference to `self` but instead define some kind of helpful trait.  Associated functions are often used in constructors to return a new instance of a struct.  Notice how in the **./struct_example** `impl` block there is a `square` function which takes in a length and returns an instance of rectangle that has the same height and width.  Associated functions are called using the `::` syntax with the struct name - they are namespaced by the struct.

It is permitted in Rust to have multiple `impl` blocks defined for the same struct however it is unnecessary in most cases.

### Chapter 6
Enums, short for *enumerations*, allow you to define a type by enumerating its possible values.  Enums can be used to decode meaning along with data but a particular type of enumeration, *option* can be used to express the presence or absence of a value.  Rust's enums are most similar to *algebraic data types* in functional languages.

#### Defining an Enum
Enum values can only be one of the variants.  To define an enum use the keyword `enum` followed by a name and `{` `}`.  Within the brackets go each variant listed as comma separated values.  To create an instance of an enum, use namespace referencing.

An application of an enum might be to wrap data concisely in an enum over a struct.  Data can be directly attached to an enum rather than a struct.  Different variants in an enum can house different data and different amounts of data.  For example look at an example to store IPv4 addresses vs IPv6 addresses:
```
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
}
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
Another similarity between enums and structs is that an enum can have an `impl` field which allows it to define its own methods.

##### The `Option` Enum and Its Advantages Over Null Values
The `option` type is used to describe the situation in which a value could be either something or nothing.  This can be very useful in error handling at compile time.  Rust does not support nulls but it does have an enum that can encode the concept of a value being present or absent.  This particular enum is `Option<T>` and is defined in the standard library as:
```
enum Option<T> {
    Some(T),
    None,
}
```
The Rust compiler will prevent one from performing operations on conflicting types of `Option\<T\>` and `T`.  Only when we deal with an `Option<T>` does the compiler have to worry about not having a value in which case the compiler will ensure that all outcomes are handled.  In order to work with an `Option<T>` and a `T`, you must convert `Option<T>` to `T` which helps catch one of the most common errors when using null which is assuming that something is there when it isn't.

#### The `match` Control Flow Operator
Rust includes support for a powerful control flow operator called `match` which allows one to compare a value against a series of patterns and then execute code based on which pattern matches.  The difference between an `if` expand a `match` is that `if` can only check for boolean values but a `match` can match any type.

The arms in a match are composed of two parts: the pattern and some code.  The `=>` is used to distinguish between these two parts.

To get the inner `T` out of an `Option<T>`, a `match` can be used.  For example:
```
fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```
Matches, as in most functional programming languages, are exhaustive.  This means that every possible outcome must be handled by a match case.  This is especially obvious why trying to use `match` with an `Option<T>` in that we cannot neglect to check for `None` and/or `Some`.

Also in parallel with other functional programming languages, a `match` can check for 'all other possible values' using a `_`.  One downside to the `match` statment is that it is a bit wordy when only one value needs to be included in the body of the `match`.  To solve this issue, Rust introduces the `if let` control flow.

#### Concise Control Flow with `if let`
The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match one pattern while ignoring the rest.  Consider the standard way to accomplish matching one value in a typical `match` statement:
```
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three!"),
    _ => (),
}
```
The alternative, less bulky way of expressing this using an `if let` is described below:
```
if let Some(3) = some_u8_value {
    println!("three");
}
```
While an `if let` is considerably less wordy, it also does not come with the exhaustive checking that `match` does.  It is possible however to include an `else` with an `if let` although this is not something that is enforced at compile time.

### Chapter 7 - Packages, Crates and Modules
Rust has a number of features related to scopes.  This is sometimes called the "module system" but it covers more than just modules:
* *Packages* are a Cargo feature taht let you build, test and share crates
* *Crates* are a tree of modules that produce a library or executable
* A *path* is a way of naming an item such as a struct, function or modules

#### Packages and Crates for Making Libraries and Executables
A summary of packages and crates
* A *crate* is a binary or library
* The *crate root* is a source file that is used to now how to build a crate
* A *package* has a *Cargo.toml* that describes how to build one or more crates.  At most one crate in a packate can be a library

When the command `cargo new ...` is issued, a *package* is being created.  Looking at the *Cargo.tomli* file, it is apparent that there is no mention of **src/main.rs**.  This is something Rust handles implicitly as Cargo's conventions are such that if there is an **src** directory containing **main.rs** in the same directory as the package's **Cargo.toml**, Cargo knoew that this package contains a binary crate with the same name as the package where **src/main.rs** is its *crate root*.  Another convention is that if the package directory contains **src/lib.rs**, the package contains a library crate with the same name as the package and **src/lib.rs** is its crate root.  The crate root files are passed by Cargo to `rustc` which actually builds the library or binary.

A package contains zero or one library crates and as many binary crates as one would like.  There must always be at least one crate (binary or library) in a package.  A package can containg both **src/main.rs** and **src/lib.rs** in which case there are two crates (a library and a binary) which both have the same name.  A package can have multiple binary crates by placing files in the **src/bin** directory where each file will be a seperate binary crate.

#### The Module System to Control Scope and Privacy
Modules allow for the organization of code intro groups.  To define a module, use the keyword `mod` followed by the name of the module.  As mentioned previously, either of the files **src/main.rs** or **src/lib.rs** are referred to as *crate roots*; this naming convention comes from the fact that the contents of either of these two files form a module named *crate* at the root of the crate's module tree.

##### Paths for Referring to an Item in the Module Tree
If we want to call a function, we must know it's *path*.  Path is very similar to name except that when thinking about path one often things of a filesystem.  A path in Rust can take on two forms:
* An *absolute path* starts from the crate root using the crate name or literal `crate`
* A *relative path* starts from the current module and uses `self`, `super` or an identifier in the current module

Both absolute and relative paths are followed by the (`::`) syntax.

##### Modules as the Privacy Boundry
Modules can be used to better organize a program but they can also be used to create privacy boundries in Rust.  If an item like a function or struct should be private, simply put it into a module.  Here are Rust's privacy rules:
* All items (functions, methods, structs, enums, modules, and constants) are private by default
* The `pub` keyword can be used to make an item public
* Private code defined within a module that is not a child of the current module is off-limits
* Any code defined in an acecstor module is available to use

Items that do not have the `pub` keyword are private as you look 'down' the module tree but items without the `pub` keyword are public as you look up the tree from the current module.

When making a module public, the items within that module are not automatically made public.  Notice that when defining a module within a module, all of the modules within the current module are able to see new modules.  Essentially think of how files within the same directory are locally available.

`super` is another important keyword that allows for a module to refer to an item in its parent module.  Here is an example of `super` being used:
```
mod instrument {
    fn clarinet() {
        super::breath_in();
    }
}

fn breath_in() {
    ...
}
```
##### Using `pub` with Structs and Enums
Structs and enums can be made public in a way that is similar to what we've seen with modules and functions.  Adding the `pub` modifier to a struct makes the struct public but the struct's fields are still private.  An example of public and private fields in a struct can be found in **./pub_example**.  While you are able to create getters and setters for fields within a struct within a module, if you try and access a non-`pub`lic field from a seperate method, an error will occur.

In contrast to a struct, if you make an enum public, all of its variants are made public as well.  An example of this can be seen in **./pub_example**.

##### The `use` Keyword to Bring Paths into a Scope
To avoid having to use long and unecessary relative and absolute paths, the `use` keyword can be used to bring paths into scope.  An example of `use` follows:
```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            ...
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```
Using an absolute path, starting from `crate::`, is perfectly valid.  The alternative, using a relative path, is a little different than simply bringing in a namespace using a relative path as one must use the `self::` prefix.

Often times it makes more sense to use the absolute path when using `use` as it is more common for code to move around within a project than the file hierarchy to change.

As a general guideline, it is more straightforward to include the parent module when referring to a function that has been made visible by a `use`.  For example:
```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            ...
        }
    }
}

use crate::sound::instrument::clarinet;

fn main() {
    clarinet();
    clarinet();
    clarinet();
}
```
Versus:
```
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            ...
        }
    }
}

use crate::sound::instrument;

fn main() {
    instrument::clarinet();
    instrument::clarinet();
    instrument::clarinet();
}
```
The second example makes it clear that `clarinet()` is not a locally defined function.  This convention is not true for structs, enums and other items however.  In the standard library, the `HashMap` is referred to with the idiomatic `use`.  For example:
```
use std::collection::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
In Rust, you can also rename a function namespace using the `as` keyword.  One can also use nested paths to eliminate vertical space in a program by bringing in multiple names from the same namespace at a time.  For example:
```
use std::cmp::Ordering;
use std::io;
```
Becomes:
```
use std::{cmp::Ordering, io};
```
To bring all publicly defined items in a path into scope, one can use the path followed by the `*` operator.

##### Seperating Modules into Different Files
To break up long programs, it is possible to move modules into seperate files.  To do so, name the file the same name as the module and then when using the module, 'forward declare' it with something similar to: `mod <name>;` where the `;` instead of `{...}` specify to Rust that the module's contents are in another file.  To have nested modules, create nested directories with module names.

### Chapter 8
Rust's standard library includes a number of very useful data structures that are called *collections*.  A collection is different from other data types in that a collection can represent multiple values.  Additionally, unlike built in arrays and tuples, collections require heap storage.

#### Storing Lists of Values with Vectors
A vector allows for the storage of more than one value in a single data structure that places items sequentially in memeory.

##### Creating a New Vector
To create a new, empty vector, call the `Vec::new` function like so:
```
let v: Vec<i32> = Vec::new();
```
We need to specify the type of values we will be inserting because we are initializing an empty collection so the compiler cannot possibly infer the type at this point.

Creating vectors that contain some values to start with is common enough that Rust supports a macro to do this:
```
let v = vec![1,2,3];
```
In this case, Rust is able to infer that the vector is composed of `i32` values so the type annotation is not necessary.

##### Updating a vector
To create a vector and then add elements to it, the `push` method comes in handy:
```
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```
##### Dropping a Vector Drops its Elements
As is the case for any struct in Rust, when a vector goes out of scope it is freed which in turns frees the elements inside.

##### Reading Elements of Vectors
A vector can be read in two ways: using indices or with the `get` method.
```
let v = vec![1,2,3];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third);
    None => println!("There is no third element");
}
```
Notice here that the `get()` method returns an `Option<&T>`.  The difference between using `get()` and indexing is that indexing a value that is out of bounds will cause the program to panic whereas using `get()` will simply return `None`.

##### Iterating over the Values in a Vector
Using a for loop:
```
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```
Using a mutable reference:
```
let mut v = vec![100, 32, 57];
for i in *mut v {
    *i += 50;
}
```
##### Using an Enum to Store Multiple Types
Vectors can only store items that are the same type.  One workaround for this is that the variants of an enum are defined under the same enum type.
```
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec! [
    SpreadSheetCell::Int(3),
    SpreadSheetCell::Text(String::from("Blue")),
    SpreadSheetCell::Float(10.12),
}
```

#### Storing UTF-8 Encoded Text with Strings
Strings in Rust are implemented as a collection of bytes.

##### What is a String?
Rust only has one string type in the core language which is the string slice `str` that is usually used as a borrowed value in the form `&str`.  The `String` type which is provided by Rust's standard library rather than coded into the core language is growable, mutable, owned and a UTF-8 encoded string type.

##### Creating a new String
As was the case with vectors, to create a new `String`, the `new` function is available:
```
let mut s = String.new();
```
It is also possible to create a `String` from a string literal.  This can be achieved with the `to_string` method that is available to all types that implement `Display`:
```
let data = "initial content";
let s = data.to_string();
// OR
// let s = "initial content".to_string();
```
As has been used before, the `String::from` method is also common:
```
let s = String::from("initial content");
```
##### Updating a String
A `String` can be grown and its contents can be changed as a vector can.  To push more data into a `String`, the `+` operator or the `format!` macro can be used to concatenate `String` values.  Here is an example of the `push_str` method being used to append a string slice:
```
let mut s = String::from("foo");
s.push_str("bar");
```
There is also another common `String` method called `push` which will append a single character to the end of a `String`.

##### Concatenation with the `+` Operator or the `format!` Macro
One way to concatenate two existing strings is with the `+` operator:
```
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```
Notice here though that the value of `s1` can no longer be used as ownership will be transferred.  The `+` operator will implicitly invoke the `add` method which has a signature that is defined with concrete types as:
```
fn add(self, s: &str) -> String {
```
Notice here that the second parameter is a reference to the second string but we are passing in a `&String` rather than a `&str`.  Rust is able to perform coercion to coerce `&String` into `&str`.  The first parameter however is taken by the `add` method.  Ultimately, instead of both strings being copied and creating a new one, the statement (`let s3 = s1 + &s2;`) will take ownership of `s1` and append a copy of the contents of `s2` and then return ownership of the result.

The `+` operator can dilute the clarity of a textual expression in which case it may make more sense to take advantage of the `format!` macro:
```
let s1 = String::from("tic");
let s1 = String::from("tac");
let s1 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```
Format will return a `String`.

##### Indexing into Strings
Indexing into a string is common to most languages but Rust will throw an error if this is attempted.  To find out why, pay attention to the next subsection.

##### Internal Representation
A `String` is a wrapper over a `Vec<u8>`.  Take a look at the length of some properly encoded UTF-8 strings:
```
let len = String::from("Hola").len();
```
The value of `len` here is going to be 4 meaning the vector is storing 4 bytes.  Each letter takes up 1 byte when encoded to UTF-8.  Compare this to the next example:
```
let len = String::from("Здравствуйте").len();
```
Here, while it appears that there are 12 characters, it takes 24 bytes to encode this UTF-8 `String`.

##### Bytes and Scalar Values and Grapheme Clusters! Oh My!
There are three relevant ways to look at string from Rust's perspective:
* As bytes
* As scalar values
* As grapheme clusters (essentially 'letters')

Looking at the Hindi word " नमस्ते , this can be broken apart into the three types mentioned above.  As a vector of u8 values, this Hindi word appears as:
```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
```
This representation is 18 bytes and is ultimately how the computer stores data.  Looking at the Unicode scalar values - which is Rust's `char` type - one sees:
```
['न', 'म', 'स', '्', 'त', 'े']
```
There are six `char` values here but the second to last and fourth to last are not actually letters but rather are diacritics that give meaning to actual letters.  What one would call the letters in this Hindi word are actually:
```
["न", "म", "स्", "ते"]
```
Rust provides different ways of interpreting the raw string data that computers store so that programmers can choose the representation that best suits them no matter what human language the text is in.

Another reason that Rust does not allow for indexing into a `String` to get a character is that indexing operations are expected to be constant time operations but it would not be possible to guarantee that performance with a `String` as Rust would have to walk through the contents from the beginning to the index specified in order to determine how many valid characters were there.

##### Slicing Strings
Although Rust does not provide a way to index into a `String` with a single index number, Rust does support using a range to create a string slice containing particular bytes:
```
let hello = "Здравствуйте";
let s = &hello[0..4];
```
In this example, `s` will be a `&str` that contains the first 4 bytes of the string.  Earlier it was mentioned that each of these characters is represented using 2 bytes so `s` will take on the value `Зд`.  If one was to try and take a string slice of size less than 2 bytes, Rust would panic citing that `'byte index 1 is not a char boundry'`.

##### Methods for Iterating Over Strings
While it is not as simple to access particular elements of a `String` as one might like, there are ways of accessing elements in a string.

The `chars` method for example separates out and returns the `char`s making up the Unicode scalar values - in the example of the Hindi word this includes the diacritics.  A simple for loop can then be used to iterate over each `char`:
```
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```
This will output:
```
न
म
स
्
त
े
```
As an alternative to `chars`, the `bytes` method can be used to return the values in the `vec<u8>`:
```
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

#### Storing Keys with Associated Values in Hash Maps
Another common collection is the *hash map*.  The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`.  A hashing function is used to determine how to place keys and values into memory.

##### Creating a new Hash Map
To create a new hash map, use the `new` function.  Add elements using the `insert` function.

An alternative method for creating a new hash maps utilizes the `zip` and `collection` methods to gather data into a number of collection types include `HashMap`s.  For example, given a vector of team names and a vector of scores, one could first `zip` to create a vector of tuples and then `collect` to turn the vector into a hash map (See in **./hash_map_examples**).

The type annotation of `HashMap<_, _>` is needed here as `collect` is able to return multiple collection types so one must be specific.  The types of keys and values however can be inferred by the types of the tuples within the zipped vector.

##### Hash Maps and Ownership
For types that implement the `Copy` trait such as `i32`, values are copied into a hash map.  For owned values such as `String`s, the values are moved into the hash map and subsequently owned by the hash map.
```
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
```
##### Accessing Values in a Hash Map
The `get` method proves a handy way to get a value out of a hash map.  The `get` method will return an `Option<&V>` as there is no guarantee that a key will have an associated value in a hash map.

To iterate over each key/value pair, a similar method to looping over `vectors` is used:
```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value;
}
```
##### Updating a Hash Map
There are three main ways to handle an incoming new value when an already existing value is stored in a hash map:
* Disregard the old value and replace it with the new value
* Disregard the new value and leave the old value
* Combine the old and new values

To overwrite a value, simply insert a key/value pair for which there is already a key/value pair

To only insert a value if a key has no value already there is a special API within hash maps called `entry`.  `entry` will take a key as a parameter and the return value will be an enum called `Entry` that represents a value that might or might not exist:
```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(10);

println("{:?}", scores);
```
The `or_insert` method on `Entry` will return a *mutable* reference to the value for the corresponding `Entry` key if that key exists and will insert the parameter as the new value otherwise.

To update a value based on the old value, the `or_insert` method can be reused.  Remember that the `or_insert` method will return a mutable reference (&mut V) to the value for a given key.  Take a look at the following example for more context:
```
use std::collections::HashMap;

let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```
##### Hashing Functions
The hashing function that Rust uses by default is called the SipHash hashing function.  It is cryptographically strong and provides greater security at the price of slightly worse performance.

### Chapter 9
Many times, Rust will ensure that a programmer handle errors and take action before code will compile.  There are two fundamental groups of errors in Rust:  *recoverable* and *unrecoverable errors*.  For a recoverable error, it is common to allow the program or user to try an action again however for an unrecoverable action, there is usually a bug at play in the program (reaching for an element beyond the bounds of an array).

Rust does not distinguish between the two types of errors and instead has the type `Result<T, E>` for recoverable errors and the `panic!` macro for unrecoverable errors.

#### Unrecoverable Errors with `panic!`
When a `panic!` macro is encountered, a program will print a failure message, clean up the stack and quit the program.  These types of errors usually occur when it is not clear to the programmer how to handle an error.

When unwinding the stack there is a lot of work that needs to be done.  It can be decided by the programmer to simply **abort** a program instead of clean up the stack at which point memory used by the program will be cleaned up by the OS.  To keep a binary small, one can use the abort feature with `panic!`.  To set this option, include in the appropriate `[profile]` section in the *Cargo.toml* file: `panic = 'abort'`.  For example:
```
[profile.release]
panic = 'abort'
```
When a `panic!` is encountered, it is possible to run a backtrace to see a list of all the functions that were called to get to the error.  The way to interpret a backtrace is to start at the top and read until a file written by the user is encountered.  To use a backtrace, run the Rust program with the `RUST_BACKTRACE=1` flag:
```
RUST_BACKTRACE=1 cargo run
```
To use a backtrace, the debug symbols in a program must be set which they are by default when the release flag is not set.

#### Recoverable Errors with `Result`
Remember from chapter 2 that there is a `Result` enum defined with two variants: `Ok` and `Err`.  This enum is specifically written as `Result<T, E>` where `T` is a generic representing the type of the value that will be returned in a success case with `Ok` and `E` is the type of the error that will be returning in the failure case with the `Err` variant:
```
enum Result<T, E> {
    Ok<T>,
    Err<E>,
}
```
One common error recovery is to create a file when trying to access one that does not currently exist.  This example follows:
```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
        other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}
```
The type of the value that `File::open` returns inside the `Err` variant is `io::Error` which is a struct provided by the standard library.  This struct has the method `kind` that can be used to get an `io::ErrorKind` value.  The enum `io::ErrorKind` is provided by the standard library and has variants representing the different kinds of errors that might result from an io operation.  This example only tries to match an `ErrorKind::NotFound` which indicates that the file being opened does not exist yet.  Here there is an outer `match` on `f` as well as an inner `match` on `error.kind()`.

##### Shortcuts for Panic on Error
Using `match` to catch particular errors is a bit verbose and does not completely communicate intent well.  The `Result<T, E>` type has many helper functions defined with it, `unwrap` is one of these methods.  `unwrap` is implemented just like `match` written in the previous example in that if `Result` returns a value that is `Ok` `unwrap` will return the value inside of `Ok` but if the variant returned is `Err`, `unwrap` will call `panic!` implicitly.  Here is an example of unwrap:
```
use std::fs::file;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
Another method, `expect` is very similar to `unwrap` except that it allows the user to specify the `panic!` message.:
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```
##### Propagating Errors
When writing a function whose implementation calls something that might fail, instead of handling the error within the function being called, one can return the error to the calling code so that a decision can be made about what do with the error.  This is known as *propogating* the error and gives more control to the calling code.  An example of propagation follows:
```
std::io::{self, Read};
std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```
This function is annotated to return a `String` in the case of success and an `io:Error` in the event of failure.  Notice that the first `return` statement is necessary as it is not the last line and hence will not be implicitly returned whereas the last match will implicitly return so that `Err(e)` does not need to be explicitly returned.

Rust provides a shortcut for propagating errors: `?`.  The following is a rewritten version of the above code using the `?` operator:
```
use std::io::{self, Read};
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
The `?` works by implicitly using `return` to return an error if one is encountered or will otherwise return the value inside of the `Ok`.

The `?` operator can only be used in functions that have a return type of `Result` because it is defined to work in the same way as the `match` expression defined higher up.  The part of the `match` that requires a return type of `Result` is `return Err(e)` so the return type of the function must be a `Result` to be compatible with this `return`.  The main function by default will return a `()` which is not a `Return` and will therefore not work with the `?` operator automatically.

##### To `panic!` or Not to `panic!`
Calling `panic!` will crash a program and not give any calling code the opportunity to make a decision about what to do in the case of an error.  For this reason, it is commonplace to use `Result` as the default return type when defining a function that may fail.

There are however some specific cases in which it is more appropriate to `panic!` such as in prototype code and tests.

##### Examples, Prototype Code, and Tests
It is understood that using `unwrap` which could cause a program to panic is a placeholder for the way one would want an application to handle errors which could differ based on what the rest of the code is doing.

The `unwrap` and `expect` methods are very handy when prototyping before one is ready to decide how to handle errors.  They leave clear marks in a program about where to return to to make error calls more robust.

If a method call fails in a test, the whole test should fail even if the method isn't the functionality being tested.  As `panic!` is how a test is marked as failed, calling `unwrap` or `expect` is exactly what should happen.

##### Cases in which You Have More Information Than the Compiler
There are some cases where the compiler is unable to see that a particular outcome is logically impossible.  Using some methods will automatically trigger a possibility of failure but in certain hard-coded operations the programmer can know that there is no possible failure:
```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```
In this example, `parse` will require the possible variant return type `Err` as the return value of `parse` is a `Result` value and the compiler cannot see that this string will *always* be valid.

##### Guidelines for Error Handling
It's advisable to have code panic when it is possible that it  could end up in a bad state.  A bad state here is defined as some situation in which an assumption, guarantee, contract or invariant has been broken such as when invalid values, contradictory values, or missing values are passed to your code plus one or more of the following:
* The bad state is not something that is *expected* to happen occasionally
* The code after this point needs to rely on not being in this bad state
* There is no good way to encode this information in the types being used

Calling `panic!` may be valid when code is passed a value that does not make sense and the best choice may be to alert the person using that particular library of the bug in their code so it can be fixed during development.  Similarly, `panic!` is often appropriate if one is calling an external code that is out of the hands of the programmer and it returns an invalid state that cannot be fixed easily.

When failure is expected, it is better to return a `Result` than to make a `panic!` call.

### Chapter 10
Generics are abstract stand-ins for concrete types or other properties.  When wring code, one can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

#### Generic Data Types
Generics can be used to create definitions for items like function signatures or structs which can be used with many different concrete data types.  The first examples will be for functions, structs, enums and methods using generics.

##### In Function Definitions
When defining a function that uses generics, generics are placed in the signature of the function where a data type would usually be specified.  A named parameter should go in the function signature so that the compiler knows what the name means:
```
fn largest<T>(list: &[T]) -> T {
```
This definition is read as: "the function `largest` is generic over some type `T`.  This function has one parameter named `list` which is a slide of values of type `T`.  The `largest` function will return a value that is of the same type `T`.

Initially, the `>` operator is not robust enough to operate on a generic type.  The compiler error mentions `std::cmp::PartialOrd` which is a  *trait*.  For now, the error is indicating that the `>` operator might not work for all types that `T` could take on.  Because this function compares values of type `T` in the body, only types whose values can be ordered can be used.  To enable comparisons, the standard library has the `std::comp::PartialOrd` trait that can be implemented on types.  To specify that a generic type has a particular train, refer to the "Trait Bounds" section.

##### In Struct Definitions
Structs can be defined to use ta generic type parameter in one or more fields using the `<>` syntax.  The following is an example defining the `Point<T>` struct to hold `x` and `y` coordinate values of any type:
```
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let integer = Point{ x: 5, y: 10 };
    let float = Point { x: 1.0. y: 4.0 };
}
```
The syntax for defining generics in a struct is similar to that used for function definitions.  Notice here that `T` can represent any data type however the type of `x` and `y` must match or the program will not compile.

To define a struct in which `x` and `y` are different types, two generic parameters can be specified as generic parameters.  For example:
```
struct Point<T, U> {
    x: T,
    y: U,
}
```
##### In Enum Definitions
As was done with structs, enums can be defined to hold generic data types in their variants.  A good example of a generic enum is the `Option<T>` enum defined as:
```
enum Option<T> {
    Some(T),
    None,
}
```
##### In Method Definitions
It is possible to implement methods on structs and enums and use generic types in their definitions too.  Simply include the generic name after the `impl` keyword as well as after the `struct` name:
```
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```
##### Performance of Code Using Generics
Rust implements generics in such a way that there is no runtime slowdown when using generic types rather than concrete types.

Rust accomplishes this using *monomorphization* of the code that is using generics at compile time.  Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

#### Traits: Defining Shared Behavior
A *trait* tells the Rust compiler about functionality that a particular type has and can share with other types.  Traits are used to define shared behavior in an abstract way.  Trait bounds can be used to specify that a generic can be any type that has certain behavior.  A trait is similar to an *interface* in other languages although there are some important differences.

##### Defining a Trait
A type's behavior consists of the methods that can be called on that type.  Different types share the same behavior if all the same methods can be called on all the types.  Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

A trait can be defined using the `trait` keyword followed by the trait's name.  Inside the body of the trait, the method signature is defined describing the behavior of the types that implement this trait.

After the method signature, instead of providing the implementation within curly brackets, a semicolon is used.  Essentially a forward declaration is being written and each type implementing this trait is responsible for providing its own custom behavior for the body of the method.  The compiler will enforce that any type that has the implemented trait will have the trait defined with this signature exactly.

A trait can have multiple methods within its body: the method signatures are listed one per line and each ends in a semicolon.

##### Implementing a Trait on a Type
Implementing a trait on a type is similar to implementing regular methods.  The difference is that after `impl`, the trait name is written followed by `for` and then the name of the type the trait is being implemented for.  Within the `impl` block, the method signatures that are defined in the trait are filled out.

A trait can only be implemented on type if either that trait or type is local.  Traits from the standard library can easily be implemented on custom made types and custom made traits can easily be implemented on standard types.  External traits cannot be implemented on external types.  For example, `Display` cannot be implemented on `vec<T>` within a custom crate as both are defined in the standard library.  This restriction is part of a property of programs called *coherence* and is more specifically the *orphan* rule which ensures that the parent type is present.  This prevents two crates from implementing the same trait on the same type causing confusion for Rust over which trait to actually use.

##### Default Implementations
Sometimes it is useful to have default behavior for some or all methods in a trait instead of requiring that all methods be implemented for every type.

To define a default implementation, create a non-empty function inside of the trait.  Then define an empty `impl` block for that specific type.  Any time a type that does not have a specific implementation of that trait, it will refer to the default implementation.  Note that it is not possible to refer to the default implementation when a specific implementation is already defined.

Default implementations can call other methods in the same trait, even if those methods don't have default implementations.  For example:
```
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```
To use this version of `Summary`, only the `summarize_author` function needs to be implemented.

##### Traits as Arguments
Traits can be used to accept arguments of many different types.  To accept any type that implements a particular trait, simply annotate the argument using `impl trait`.  For example:
```
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
##### Trait Bounds
The aforementioned `impl trait` implementation works for short examples but is syntactic sugar for a longer form.  The longer form is called a *trait bound* and looks like:
```
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
```
With a trait bound, a generic type has been annotated as some type which implements the `Summary` trait.

##### Specify Multiple Traits with `+`
To specify multiple traits on one type, use the `+` syntax:
```
pub fn notify(item: impl Summary + Display) {
```
or
```
pub fn notify<T: Summary + Display>(item: T) {
```
The `+` syntax provides easy implementation of multiple traits on a single type however it can become overwhelming to read with many types implementing many traits.  To simplify this issue, Rust provides the `where` clause:
```
fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```

##### Returning Traits
The `impl trait` syntax can be used for more than just specifying arguments, it can also be used to specify return values:
```
fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```
In this case, *any* type that implements the `Summary` trait can be returned.

It is important to note that this syntax only allows for **one** type to be returned even if the alternative return types all implement the same trait.

##### Using Trait Bounds to Conditionally Implement Methods
By using a trait bound with an `impl` block that uses generic type parameters, methods can implemented conditionally for types that implement specific traits.  For example, a struct will always implement the `new` function but will only implement `cmp_display` if its inner type implements the `PartialOrd` trait that allows for comparison *and* the `Display* trait that allows for printing.  For example:
```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
One can also implement a trait for any type that implements another trait.  Implementations of a trait on any type htat satisfies the trait bounds are called *blanket implementations* and are used frequently in the Rust standard library.  For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait.  The `impl` block in the standard library looks similar to:
```
impl<T: Display> ToString for T {
    ...
}
```
This allows for any type that implements the `Display` trait to call the `to_string` method defined by this `ToString` trait.

#### Validating References with Lifetimes
Every reference in Rust has a *lifetime*.  A lifetime is the scope for which a reference is valid.  Most of the time, lifetimes are implicit and inferred as most of the time types are inferred.  Similar to how one must annotate types when multiple are possible, one must annotate lifetimes when the lifetimes of references could be related in a few different ways.  Rust requirest that these relationships be annoted with lifetime generics to ensure the actual references used at runtime are actually valid.

##### Preventing Dangling References with Lifetimes
Preventing dangling references is one of the main goals of lifetimes in Rust.  A dangling reference is when a program attempts to reference data other than the data it's intended to reference.  For example:
```
{
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
```
In this example, while the variable name `r` exists in the outer scope, the value does not.  The error that the compiler will identify is that the lifetime of `x` is too short.  To catch this error, Rust uses a *borrow checker*.

##### The Borrow Checker
At compile time, Rust compares the size of multiple lifetimes and sees if there are gaps in lifetimes in which references are being used illegally.

##### Lifetime Annotation Syntax
Lifetime annotations don't change how long any of the references live.  Just as functinos can accept any type when the signature specifies generic type parameters, functions can accept references with any lifetime by specifying a generic lifetime parameter.  Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

Lifetime annotations must start with an apostrophe `'` and are usually lowercase and short - like generic types.  Many people use the `'a` name for lifetime generics.

##### Lifetime Annotations in Function Signatures
Similar to generic type parameters, generic lifetime parameters are stored inside angle brackets between the function name and the parameter list.  One constraint that is being specified by using generic lifetime annotations is that the references in the parameter and the return value must have the same lifetime.

Examine the `longest` function in the **./lifetime_example**:
```
fn longest<'a>(x: &'a str, y: &'a str) -? &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```
The function signature for `longest` now tells Rust that for some lifetime `'a`, the function takes two parameters which are both string slices that live at least as long as lifetime `'a`.  These lifetime constraints are ultimately communicating to the borrow checker that it should reject any values that don't adhere to the specified lifetime constraints.  In this case, that means the parameters need to last as long as the return value.

The lifetime specified in a function will be the smallest of the lifetime arguments specified.  For example:
```
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```
In this program, the lifetime of result wille the smaller of the two lifetimes of `string1` and `string2`.  Since `string2` will only last as long as the inner scope, result will not survive past the inner scope either.

##### Thinking in Terms of Lifetimes
When returning a refernce from a function, the lifetime parameter for the return type needs to match the parameter for one of the parameters.  If the reference were to match anything else it would have to be a newly created value from within the function which would mean that there's a dangling reference after the function returns as the value created within the function goes out of scope once the function finishes.

One way to fix this dangling reference issue is to instead returned an owned value.  Ultimately, the lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.  Once they are connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers to otherwise violate memry-safety.

##### Lifetime Annotations in Struct Definitions
It is possible for structs to hold references instead of only owned types.  In the event that a struct mantains a reference, a lifetime annotation needs to be added on every reference in the struct's definition.

The following example defines a struct which holds a string slice:
```
struct ImportantExceprt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.');
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExceprt { part: first_sentence };
}
```
The annotation of `ImportantExcerpt` here means that an instance of `ImportantExceprt` cannot outlive the reference it holds in its `part` field.

The `main` function here creates an instance of the `ImportantExcerpt` struct that holds a reference to the first sentence of the `String` owned by the variable `novel`.  The data in `novel` exists before the `ImportantExceprt` instance is created.  In addition, `novel` doesn't go out of scope until after the `ImportantExceprt` goes out of scope, so the reference in the `ImportantExcerpt` instance is valid.

##### Lifetime Elision
The presence of lifetime annotations has decreased since the early versions of Rust as people realized there are deterministic patterns to how lifetime annotations are defined when using them so the compiler was made smarter to pick up on these patterns.  The patterns programmed into Rust's analysis of references are called *lifetime elision rules*.  These patterns aren't rules for programmers to follow but rather are particular cases that the compiler will consider.  If code fits these cases one does not need to write lifetimes explicitly.

The compiler will not make guesses about lifetimes of references but it can provide error messages which can be resolved by adding the lifetime annotations that specify how the references relate to each other.

Lifetimes on function or method parameters are called *input lifetimes* and lifetimes on return values are called *output lifetimes*

The compiler uses three rules to figure out what lifetimes references have when there aren't explicit annotations.  The first rule applies to input lifetimes, the second and third apply to output lifestimes.  If the compiler gets to the end of the three rules and there are still references for which it can't figure out lifetimes, the compiler will return an error.

These rules apply to `fn` as well as `impl` blocks.

###### Rule 1
Each parameter that is a reference get is own lifetime parameter.  This means a function with k references will receive k lifetimes.

###### Rule 2
If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.

###### Rule 3
If there are multiple input lifetime parameters but, one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.

##### Lifetime Elision Rule Examples
For this example, the function annotation will start off as:
```
fn first_word(s: &str) -> &str {
```
After applying the first rule which specifies that each parameter gets its own lifetime, the function signature will implicitly become:
```
fn first_word<'a>(s: &'a str) -> &str {
```
The second rule applies as there is only one input lifetime so the output lifetime parameter becomes the input lifetime parameter:
```
fn first_word<'a>(s: &'a str) -> &'a str {
```
At this point the compiler is satisfied as each reference in the function have associated parameters.

Another example starts as:
```
fn longest(x: &str, y: &str) -> &str {
```
Using the first rule, this signature becomes:
```
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```
The second rule does not apply as the are two references as parameters.  The third rule also does not apply as `longest` is a function rather than a method so there is no `self` to be used.  There are no further rules to check but the function signature is not fully annotated meaning the compiler will require additional information.

##### Lifetime Annotations in Method Definitions
Lifetime names for struct fields are always declared after the `impl` keyword as well as after the struct's name:
```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```
An example of the third lifetime elision rule in action follows:
```
impl<'a> ImportantExcerpt<'a> {
    fn accounce_and_return_part(&self, accouncement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```
In this example, there are two input lifetimes so both get their own lifetime.  As one of the parameters is `&self`, the return type also gets the lifetime of `&self`.

##### The Static Lifetime
One special lifetime in Rust is the `static` lifetime which denotes the entire duration of the program.  By default, all string literals have the `static` lifetime:
```
let s: &'static' str = "I have a static lifetime";
```
The text in a string is stored directly in the binary of the program.

Often times, `static` is not actually necessary but seems to be as the result of a dangling reference or a mismatch of available lifetimes.

#### Generic Type Parameters, Trait Bounds, and Lifetimes Together
An example of the syntax for specifying generic type parameters, trait bounds and lifetimes all in one function follows:
```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Accouncement! {}", ann);
    if x.len() > y.len() {
        x
    else {
        y
    }
}
```
This is an adaptation of the `longest` function previously contained in the **./generic_example**.  The difference with this function is the presence of an extra parameter, `ann` which is generic type `T` which can be filled by any type that implements the `Display` trait as specified by the `where` clause.  The reason the `Display` trait is specified is that whatever value `ann` holds will be printed before the longest of the two strings is computed.  Because lifetimes are a type of generic, the declarations of the lifetime parameter `'a` and the generic type parameter `T` go in the same list inside the angle bracket after the function name.

### Chapter 11
Rust provides a number of built in testing applications to make it easier to perform test on Rust programs.

#### How to Write Tests
Tests are Rust functions that verify that non-test code is functioning in the expected manner.  The bodies of test functions typically perform these actions:
* Set up any needed data or state
* Run the code you want to test
* Assert the results are what you expect

Some of the features Rust provides specifically for writing tests include the `test` attribute and the `should_panic` attribute.

##### The Anatomy of a Test Function
A test is Rust is a function that is annotated with the `test` attribute.  Attributes are metadata about pieces of Rust code.  One example is the `deriv` attribute which was used in structs earlier on.  To change a function into a `test` function simply add: `#[test]` on the line before the `fn`.

When making a new library project with Cargo, a test module with a test function in it is automatically generated.

To create a new library project called adder:
```
$ cargo new adder --lib
```
It is important to include the `#[test]` line in a test program as there can be functions that are not test functions within a testing program.  The measured statistic is only available in nightly Rust.

##### Checking Results with the `assert!` Macro
The `assert!` macro is useful in that it ensures some condition in a test evaluates to `true`.  If the value in an `assert!` evaluates to `true`, `assert!` does nothing and the program continues.  If the value is computed to be `false` then the `assert!` macro calls the `panic!` macro.

##### Testing Equality with the `assert_eq!` and `assert_ne!` Macros
A common way to test functionality is to compare the result of the code under test to the value you expect the code to return to make sure they're equal.  This can be accomplished using the `assert!` macro and passing it an expression that uses the `==` operator.  This is such a common test however that there is a special macro to handle just this: `assert_eq!` and `assert_ne!`.  These two macros will conveniently print the two values being compared should the assert macro not be satisfied.

##### Adding Custom Failure Messages
It is possible to specify the failure message that is printed when an assert macro fails.  Any arguments passed in addition to the `assert!` or `assert_eq!` or `assert_ne!` are automatically passed along to the `format!` macro so a format string that contains a `{}` can be passed.

##### Checking for Panics with `should_panic`
In addition to checking that a program returns correct values, it is important to check that a program correctly handles errors as well.

To check that a function fails and panics when it should, add the `should_panic` line to a test function.  This attribute ensures that a test passes when the code inside the function panics - the test will fail if a panic is not encountered.

Checking for a panic can be tricky as the error message displayed when a message does *not* panic when it was supposed to tend not to be very helpful.  Another issue is that the function causing the program to panic might not be the one that is intended to fail in which case a false positive is encountered.  To avoid the latter issue, one can use the `expected` value to check for the substring of an error message:
```
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
...
```
Notice that the error message being expected is simply a *substring* and does not need to be the entire message.

##### Using `Result<T, E>` in tests
Another common way to check for errors aside from using some kind of assert is to return the variants of the `Result<T, E>` trait.

#### Controlling How Test Are Run
`cargo test` will compile code and run the resulting binary just as `cargo run` does.  There are command line flags that can be utilized to change the way in which `cargo test` operates.  By default, `cargo test` will run all tests in parallel and capture the output generated during test runs.  Some of the command line arguments will go to `cargo test` and some will go to the resulting binary.  To specify which go where, the `--` separator is used.  The ones going to `cargo test` come before the `--` and the ones going to the binary come after the `--`.

##### Running Tests in Parallel or Consecutively
The default behavior of `cargo test` is to run in parallel using multiple threads.  One needs to ensure that their tests will not overlap with each other as Rust will not check for this itself.  Another option is to run things consecutively using the command:
```
cargo test -- --test-threads=1
```

##### Showing Function Output
By default, the standard output from a function will not be displayed during a test when it succeeds - failure will result in printed output.

##### Running a Subset of Tests by Name
To run only a subset of tests, enter `cargo test` followed by the name(s) of test(s) to run as an argument.

It is also possible to specify part of a test name and any test whose name matches that value will be run.

##### Ignoring Some Tests Unless Specifically Requested
To avoid running particular tests, add the `#[ignore]` attribute after the test attribute.  It is also possible to only run those tests which have been marked as `#[ignored]` using the command:
```
cargo test -- --ignored
```

#### Test Organization
The Rust community structures tests in terms of two main categories:
* Unit tests
* Integration tests

Unit tests are small and more focused - they test only module in isolation at a time and can be used to test private interfaces.

Integration tests are entirely external to a library and use the code within a library in the same way that an external program would meaning only the public interface is used.

##### Unit Tests
As mentioned before, unit tests are used to test code in isolation to pinpoint sources of error quickly.  Unit tests are placed in the *src* directory in each file with the code that is being tested.  The convention is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`.

###### The Test Module and `#[cfg(test)]`
The #[cfg(test)]` annotation on test modules tells Rust to compile and run only the test code only when running `cargo test` (not when using `cargo build`).  This `#[cfg(test)]` annotation is important only for unit tests s they are in the same source file as the code they are testing whereas an integration test is written seperate from the code it is testing so it does not require the presence of the `#[cfg(test)]` annotation.

The attribute `cfg` stands for *configuration* and tells Rust that the following item should only be inlcluded given a certain configuration option.  In this case, the configuration option is `test` which is provided by Rust for compiling and running tests.

###### Testing Private Functions
While it is a controversial topic in many languages, Rust allows for testing programs to to test private functions.

For the following example:
```
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```
the `internal_adder` is never marked as `pub` but because test are just Rust code and the `tests` module is just another module, the `internal_adder` an be brought into a tests's scope to be called.

##### Integration Tests
In Rust, integration tests are completely external to a library.

###### The *tests* Directory
A **tests** directory is created at the top level of a project directory next to *src* for integration tests.  Cargo knows to look for integration test files in the **tests** directory at the top level.  Within this directory, as many test files as one wants can be created and Cargo will compile each file to create an individual crate.

A `use` statement should be added to the top of the the code in an integration test file within the **tests** directory as each test in the **tests** directory is a seperate crate so one needs to bring the library being tested into each test crate's scope.


Integration tests, like unit tests, can be run using the `cargo test` command.  When running `cargo test`, there are three sections of output:
* unit tests
* integration tests
* doc tests

To specifically run an integration test, simply include the name of the integration test following the `--test` flag when running `cargo test`:
```
cargo test --test integration
```

###### Submodules in Integration Testing
As more integration tests are added, it might be worthwhile to create more than one file in the **tests** directory to help organize them.  One way to organize is to group tests by the functionality that they are testing.

Treating each integration test file as its own crate is useful to create seperate scopes that are more like the way end users will be using these test crates.  This also means that files in the **tests** directory don't share the same behavior as the files in **src**.

To create helper functions that will not show up in the testing suite, create a **mod.rs** file.  Rust's compiler will understand the distinction and avoid testing functions within **mod.rs**.

###### Integration Tests for Binary Crates
It a project only contains **./src/main.rs** and no **./src/lib.rs** there can be no integration tests in the **tests** directory which call functions defined in the **./src/main.rs** with a `use` statement.  Only library crates expose functions that other crates can use; binary crates are meant to run independently.

For this reason, most Rust projects have a straight forward **./src/main.rs** file which calls on logic that is defined in the **./src/lib.rs** file.  using this structure, integration tests can test the library crate with a `use` to make the important functionality available.

### Chapter 12
This chapter will serve as practice for many of the skills acquired so far as well as explore some more standard library features.  The final product for this chapter will be a command line tool that will interact with file and command line input/output.

This project will be an implementation of the `grep` (**g**lobally search a **r**egular **e**xpression and **p**rint).  Grep takes in a file name as well as a string to find within that file.  It searches every line in that file to find the word specified and if it succeeds it will print those lines with the specified word.

#### Accepting Command Line Arguments
##### Reading the Argument Values
To enable this project to read from the command line, the standard library function `std::env::args` will be used.  This function returns an iterator of the command line arguments that are passed to the program when run.  The `args` function will panic if any argument contains invalid Unicode.  If invalid Unicode needs to be accepted, use the `std::env::args_os` instead.  The `args_os` function will return `OsString` values instead of `String` values.

The program's name will be the first element in the vector so it is likely that the program will begin accepting input from index 1 and on.

#### Reading a File
Reading a file can be accomplished using the standard library function `read_to_string` found in the `std::fs` module.

#### Separation of Concerns for Binary Projects
Originally, the main function took on a great deal of responsibility in that it acted alone in parsing command line arguments.

To take pressure off of the main function, a `parse_config` function is created.  This function interacts with a struct, `Config` which is defined as holding two string values: *query* and *filename*.

The signature of the `parse_config` function takes in a reference to a `String` array and returns a `Config` value.  The returned `Config` will also take ownership of the `String` values being parsed - *query* and *filename*.  In this case, the main function owns the `args` vector and passes a reference to the `parse_config` function.  Within `parse_config`, the values of `query` and `filename` are cloned which is not the most efficient solution but is relatively straight forward.

In general, Rustaceans avoid using `clone` to fix ownership problems because of runtime costs.

##### Creating a constructor for `Config`
The general purpose of the original `parse_config` method was to create a `Config` instance.  `parse_config` can be changed from a plain function to a function named `new` that is associated with the `Config` struct.

##### Calling `config::new` and Handling Errors
To handle the error of having too few arguments in a user-friendly way, main and `Config::new` have been updated to work with `Result` types.  `Config::new` will now return either an `Err` or an `Ok` variant for main to deal with.  Main now uses the `.unwrap_or_else` which allows for the user to define non-`panic!` error handling.  If the `Result` returned from `Config::new` is `Ok` then the method `.unwrap_or_else` will `unwrap` by returning the inner value `Ok` is wrapping.  If the `Result` returned is `Err` the method calls code in *closure* which is an anonymous function defined as an argument in `.unwrap_or_else`.  In this case, the inner value of `Err` will be printed as the user's error message.

##### Returning Errors from the `run` Function
To make things even better, the `run` function can be implemented that does some of the logic main was previously responsible for.  `run` can also return a `Return` type that main can interpret how it wants to allowing for more helpful error handling.  `run` now returns a unit type, `()`, as well as a *trait object* which essentially allows the return value to be anything that implements the `Error` trait.

Another interesting implementation in `run` is the move away from `.expect()` to the `?` syntax.  The `?` will return the error value from the current function to the caller instead of `panic!`ing.

The `run` function also returns `Ok` in success cases.  It may look strange to return `Ok( () )` but this ultimately is saying that `run` is used for its side effects rather than return values.

Within main there needs to be a check to see if `run` returns an `Ok` or `Err` variant.  This is handled using an `if let` statement rather than a `.unwrap_or_else()` as there is no value to unwrap since `run` returns a `()` type.

##### Splitting Code into a Library Crate
The next step in improving this program is to create a **./lib.rs** file which will house everything except for the `main` function which will remain in the **./main.rs** file.  To successfully transition, the **./main.rs** must `use` both `minigrep` and `minigrep::Config`.  The `use minigrep;` is to bring the library crate into the binary crate.  The `use minigrep::Config` is used to bring the `Config` struct into scope as well.

Within **./lib.rs** the `Config` struct, `Config::new` function, and run function must be made public.   This allows the **./main.rs** to access these functions that are by default private.

#### Developing the Library's Functionality with Test-Driven Development
After extracting the logic from a single **./main.rs** file into both a **./main.rs** and a **./lib.rs**, it is easier to write tests to validate the core functionality of the this program.

The Test-driven development (TDD) technique follows the following steps:
1. Write a test that fails and run it to make sure it fails for the reason it is expected to fail
2. Write or modify just enough code to make the new test pass
3. Refactor the code just added or modified and make sure the tests continue to pass
4. Repeat from step 1!

##### Writing a Failing Test
To begin, add the `tests` module with a test function.  The test function specifies the behavior that the `search` function should have.  The `search` function will take a query and the test to search for the query in, and will return only the lines from the test that contain the query.

The `search` function takes two arguments: `query` and `contents`.  The return type for this function is a `Vec<&str>`.  The vector of string slices that is being returned should contain string slices from `contents` and not `query` which is why there is a lifetime annotation added to become:
```
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```
The lifetimes dictate that the `Vec` is directly related to `contents` and not `query`.  The annotation also dictates that the data returned from `search` will live as long as the data passed into the `search` function in the `contents` argument.

##### Writing Code to Pass the Test
The test currently fails as an empty vector is always returned from `search`.  To fix this and implement a working `search` the following steps need to be taken:
* Iterate through each line of the contents
* Check whether the line contains the query string
* If it does, add it to the list of values to return
* if it doesn't, do nothing
* Return the list of results that match

###### Iterating Through Lines with the `lines` Method
Rust has a helpful method to handle line-by-line iteration of strings named `lines`.  The `lines` method returns an iterator.  A `for` loop can be used with an iterator to run some code on each item in a collection.

###### Searching Each Line for the Query
Strings have a helpful method called `contains` which checks if a substring exists within a string.

###### Storing Matching Lines
There needs to be a data structure capable of storing the lines containing the query string.  To handle this, a mutable vector preceding the `for` loop can be made and the `push` method can be called to store a `line` in the vector.  After the `for` loop, the vector can be returned.

###### Using the `search` Function in the `run` Function
After updating the `search` function which is only being used in the `test` module, it is time to move it to the `run` function.  Notice that the `test` works because `duct` is a substring within `productive` meaning it will return the whole line.

##### Working with Environment Variables
`minigrep` can be improved by adding an extra feature: an option for case-insensitive searching that the user can turn on via an environment variable.  This option could be enabled using a command line option that would require the user to enable it every time.  Instead, an environment variable can be used which would allow a user to set the environment variable once and have all their searches be case insensitive in that terminal session.

###### Writing a Failing Test for the Case-Insensitive `search` Function
A new `search_case_insensitive` function will be added that will be called when the environment variable is on.  Following the TDD process, the first step is to write a failing test.

##### Implementing the `search_case_insensitive` Function
The `search_case_insensitive` function has almost the same functionality as `search`.  The only difference is that `query` and `line` will be made lowercase so that case does not make a difference.

#### Writing Error Messages to Standard Error Instead of Standard Output
Originally, all output was directed to the terminal using `println!` in the **./minigrep** files.  Most terminals provide two different kinds of output:
* Standard output (`stdout`)
  * Used for general information that is normal output
* Standard error (`stderr`)
  * Used for error messages
  
The distinction may seem subtle but is important.

##### Printing Errors to Standard Error
The Rust standard library provides the `eprintln!` macro which to the `stderr` stream instead of `stdout`.  In this case, even if we were to direct the output of our program into a file, the `stderr` would not go into the file, only the `stdout` will do that.

### Chapter 13
Rust takes extensive inspiration from functional programming languages.  As defined in the Rust docs, functional programming often includes using functions as values by passing them in arguments and returning them from other functions as well as assigning them to variables for later execution.

This chapter will cover:
* _Closures_, a function-like construct that can be stored in variables
* _Iterators_, a way of processing a series of elements
* How to use these two features
* Performance of these two features

#### Closures: Anonymous Functions that Can Capture Their Environment
Rust's closures are anonymous functions which can be saved in variables or passed as first class arguments in other functions.  Unlike functions, closures can capture values from the scope in which they're defined.

##### Creating an Abstraction of Behavior with Closures
Normally we could use a variable to store a result we are going to use later.  If we are not sure we are going to use the result later however, it might not always be appropriate to calculate it.  This is where closures come in handy.

Closures can be identified by their matching pipe symbols: `|`: `|param1|` or `|param1, param2, ...|`  The value returned from the last line in the closure body will be the value returned form the closure when it is called as that line does not end in a `;` just as in a function body.

A closure will not evaluate until it is called however if we make use of the same closure twice in one context, it is preferable to not have to recompute the value.  Normally this could be accomplished by saving the return value of the closure in a variable, however closures in Rust have a built in way of accomplishing this!

Unlike functions, closures do not require explicit type annotations as closures are not part of an explicit interface exposed to users.  Closures are really only used in narrow contexts, different from functions, which allows for the compiler to reliably infer the types of parameters and the return type.  This being said, it is still possible to annotate types in closures:
```
let expensive_closure = |num: u32| -> u32 {...}
```

While a closure may not initially have any type annotations, once it is used, the initial types passed in are locked and may not change.

##### Storing Closures Using Generic Parameters and the `Fn` Traits
As mentioned previously, there is a specific way in which the value from a closure can be saved after first being computed.  This can be achieved with a struct which will hold the closure and the result value of the calling closure.  The struct will execute only if the resulting value is needed and will _cache_ the resulting value so that the rest of the code does not have to be responsible for saving and reusing the result.

To create a struct that will hold a closure, we need to specify the type of the closure, because a struct definition needs to know the types of each of its fields.  Each closure has its own unique anonymous type meaning even if two closures have the same signature, their types are still considered to be different.  To define structs, enums or function parameters that use closures, use generics and trait bounds.

The `Fn` traits are provided by the standard library.  All closures implement at least one of the traits: `Fn` `FnMut` or `FnOnce` which will be discussed in a later chapter.  As is shown in the closure_example project's **src/main.rs**, the closure `expensive_closure` is used with a parameter of type `u32` so when define a corresponding struct as:

```
struct Cacher<T>
	where T: Fn(u32) -> u32
{
	calculation: T,
	value: Option<u32>
}
```
Notice that in the above code snippet, the `value` field is of type `Option<u32>` as before we execute the closure, the `value` will be `None`.  When code using `Cacher` asks for the _result_ of the closure, the `Cacher` will execute the closure at that time and store the result within a `Some` variant in the `value` field.

One issue with the initial set up of `Cacher` in **src/main.rs** is that if the same closure is called with different arguments, Rust will `panic!`.  The reason for this is that the value of `value` is fixed at the first use in the naive implementation.  To fix this, try using a hash map in place of `value` and when retrieving `value`, look up the argument passed into the closure to see if it has already been evaluated.

Another issue is that the naive `Cacher` implementation will only accept closures which take on parameter of type `u32` and return a `u32`.  It might be desirable to cache the result of closures that take a string slice and return `usize` values, for example.  To fix this issue, try introducing more generic parameters to increase the flexibility of the `Cacher` functionality.

##### Capturing the Environment with Closures
Initially, closures seemed valuable for their anonymous function properties.  Additionally, closures have the ability to capture their environment and access variables from the scope in which they're defined which is different from how functions function.  Closures by default capture variables in the environment they are defined in, this creates memory overhead that functions will never have.

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:
* Taking ownership
* Borrowing mutably
* Borrowing immutably

these three distinctions are encoded in the three `Fn` traits as follows:

* `FnOnce` consumes the variables it captures from its enclosing scope, known as the closure's _environment_.  To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined.  The `Once` part of the name represents the fact that the closure can't take ownership of the same variables more than once, so it can only be called once.
* `FnMut` can change the environment because it mutably borrows values.
* `Fn` borrows values from the environment immutably.

When using a closure, rust infers which trait to use based on how the closure uses the values from the environment.  All closures implement `FnOnce` because they can all be called at least once.  Closures that don't move the captured variables also implement `FnMut`, and closures that don't need mutable access to the captured variables also implement `Fn`.

If a closure is to take ownership of the values it uses in the environment, the `move` keyword can be specified before the parameter list.  This technique is mostly used when passing a closure to a new thread to move the data so it's owned by the new thread.  For example:
```
fn main() {
	let x = vec![1, 2, 3];
	let equal_to_x = move |z| z == x;
	println!("can't use x here: {:?}, x);
	let y = vec![1, 2, 3];
	assert!(equal_to_x(y));
}
```

a
