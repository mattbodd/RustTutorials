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

Signed numbers are stored using two's complement representation.  Each signed variant can store numbers from **-(2^(n-1))** to **2^(n-1)**  where *n* is the number of bits that a variant uses.  In Rust, all number literals except the byte literal allow a visual separator such as `1_000`
Rust is able to handle overflow in production and will perform two's complement wrapping to mod the number by its corresponding intended size.  In development, this wrapping will not occur and signal an error instead.

Rust also has two primitives for floating points as well.  Rust supports `f32` and `f64` where the default is `f64` as on most CPU's there is not much difference speed wise between the two types of floats.

The character primitive in Rust also supports more than just ASCII.  You can represent accented letters, Chinese, Japanese and Korean characters, emojis and zero-width spaces.

##### Compound Types
*Compound Types* can group multiple values into one type.  Rust has two primitive compound types: **tuples** and **arrays**.

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

Also in parallel with other functional programming languages, a `match` can check for 'all other possible values' using a `_`.  One downside to the `match` statement is that it is a bit wordy when only one value needs to be included in the body of the `match`.  To solve this issue, Rust introduces the `if let` control flow.

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
* *Packages* are a Cargo feature that let you build, test and share crates
* *Crates* are a tree of modules that produce a library or executable
* A *path* is a way of naming an item such as a struct, function or modules

#### Packages and Crates for Making Libraries and Executables
A summary of packages and crates
* A *crate* is a binary or library
* The *crate root* is a source file that is used to now how to build a crate
* A *package* has a *Cargo.toml* that describes how to build one or more crates.  At most one crate in a package can be a library

When the command `cargo new ...` is issued, a *package* is being created.  Looking at the *Cargo.toml* file, it is apparent that there is no mention of **src/main.rs**.  This is something Rust handles implicitly as Cargo's conventions are such that if there is an **src** directory containing **main.rs** in the same directory as the package's **Cargo.toml**, Cargo knows that this package contains a binary crate with the same name as the package where **src/main.rs** is its *crate root*.  Another convention is that if the package directory contains **src/lib.rs**, the package contains a library crate with the same name as the package and **src/lib.rs** is its crate root.  The crate root files are passed by Cargo to `rustc` which actually builds the library or binary.

A package contains zero or one library crates and as many binary crates as one would like.  There must always be at least one crate (binary or library) in a package.  A package can containing both **src/main.rs** and **src/lib.rs** in which case there are two crates (a library and a binary) which both have the same name.  A package can have multiple binary crates by placing files in the **src/bin** directory where each file will be a separate binary crate.

#### The Module System to Control Scope and Privacy
Modules allow for the organization of code intro groups.  To define a module, use the keyword `mod` followed by the name of the module.  As mentioned previously, either of the files **src/main.rs** or **src/lib.rs** are referred to as *crate roots*; this naming convention comes from the fact that the contents of either of these two files form a module named *crate* at the root of the crate's module tree.

##### Paths for Referring to an Item in the Module Tree
If we want to call a function, we must know it's *path*.  Path is very similar to name except that when thinking about path one often things of a filesystem.  A path in Rust can take on two forms:
* An *absolute path* starts from the crate root using the crate name or literal `crate`
* A *relative path* starts from the current module and uses `self`, `super` or an identifier in the current module

Both absolute and relative paths are followed by the (`::`) syntax.

##### Modules as the Privacy Boundary
Modules can be used to better organize a program but they can also be used to create privacy boundaries in Rust.  If an item like a function or struct should be private, simply put it into a module.  Here are Rust's privacy rules:
* All items (functions, methods, structs, enums, modules, and constants) are private by default
* The `pub` keyword can be used to make an item public
* Private code defined within a module that is not a child of the current module is off-limits
* Any code defined in an ancestor module is available to use

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

#### Processing a Series of Items with Iterators
An iterator allows for tasks to be performed on a sequence of items in turn.  An iterator handles all of the logic responsible for iterating over each item and determining when the sequence has terminated.

In Rust, iterators are _lazy_ meaning they have no effect until methods are called which consume the iterator to use it up.  An iterator can be applied to a variety of collections.  For example, to create an iterator for a vector:
```
let v1 = vec![1, 2, 3];
let v1_iter = v1.iter();
```

Once an iterator has been created, it can be used in a variety of ways.  One common way is to use a `for` loop to iterate over all of the elements within it.

##### The `Iterator` Trait and the `next` Method
All iterators implement a trait named `Iterator` that is defined in the standard library.  The definition looks like:
```
pub trait Iterator {
	type Item;
	
	fn next(&mut self) -> Option<Self::item>;	
	...
}
```

Notice that the above definition uses new syntax: `type Item` and `Self::Item` which are used to define an `associated type` with this trait.  At a high level, this code requires that the `Iterator` trait define an `Item` type which is used in the return type of the `next` method.  In other words, the `Item` type will be the type returned from the iterator.

The `Iterator` trait only requires implementors to define one method: the `next` method which returns one item of the iterator at a time wrapped in `Some` and, when iteration is over, returns `None`.

The `next` method can be called on iterators directly.

An `iter` needs to be mutable as calling the `next` method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.  In other words, `next` will _consume_ the iterator.  Also note that the values returned from `next` are immutable references of the values in the collection being iterated over.  In order to create an iterator which takes ownership and returns owned values, use `into_iter` instead of `iter`.  Similarly, if it is desirable to iterator over mutable references, call into `iter_mut` instead of `iter`.

##### Methods that Consume the Iterator
The `Iterator` trait has a number of different methods with default implementations provided by the standard library.  Some of these methods call `next` which is why it is required to implement the `next` method when implementing the `Iterator` trait.

Methods which call `next` are called _consuming adapters_ because calling them uses up the iterator.

##### Methods that Produce Other Iterators
Other methods defined on the `Iterator` trait, known as _iterator adapters_ allow for chances to iterators to convert to different kinds of iterators.  Multiple calls to iterators can be chained to perform complex actions in more readable ways.  Keep in mind that as iterators are lazy by nature, one of the consuming adapters has to be called to get results from calls to iterator adapters.  For example:
```
let v1: vec![1, 2, 3];
v1.iter().map(|x| x + 1);
```

The above code will not actually perform the mapping as the `iter` must be called first.  To fix this issue, `collect` can be used to invoke the `iter` function.

##### Using Closures that Capture Their Environment
There is a common use of closures that involves capturing their environment using the `filter` iterator adapter.  The `filter` iterator takes a closure that takes each item from the iterator and returns a Boolean.  If the closure returns `true`, the value will be included in the iterator produced by `filter`.  If the closure returns `false`, the value won't be included in the resulting iterator.

##### Creating Our Own Iterators with the `Iterator` Trait
As stated previously, in order to extend the iterator interface to a custom type, simply implement the `next` method.  Take for example this `struct`:
```
struct Counter {
	count: u32,
}

impl Counter {
	fn new() -> Counter {
		Counter { count : 0}
	}
}
```

This defines Counter to start with an attribute, count = 0.  Now say that we want to create an iterator which will iterate until the value of Counter.count is 5.  This can be achieved with:
```
impl Iterator for Counter {
	type Item = u32;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
		
		if self.count < 6 {
			Some(self.count)
		} else {
			None
		}
	}
}
```

Iterators in Rust are an example of what is known as a _zero cost abstraction_ which means runtime performance is not lost in abstraction.

Nice quote: "In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better."

### Chapter 14
#### Customizing Builds with Release Profiles
Release profiles allow for programmers to specify different variations of compiling the same project.  Each profile is configured independently of others.  Cargo has two main build profiles: `dev` and `release`.  `dev` is triggered with: `cargo build` while `release` is triggered with `cargo build --release`.  It is also possible to specify custom profiles by simply inserting `[profile.*]`.  By default, **Cargo.toml** will include:
```
[profile.dev]
opt-level = 0
[profile.release]
opt-level = 3
```

#### Publishing a Crate to Crates.io
Crates.io will store source code which can be distributed.
##### Making Useful Documentation Comments
Using /// instead of just // will signify a documentation comment which can automatically be used automatically when creating generating HTML documentation.  These documentation comments also support Markdown.  This documentation can be generated using `cargo doc` which will implicitly use `rustdoc` and place the resulting HTML into **target/doc**.

### Chapter 15
#### Smart Pointers
A pointer is a general concept for a variable that contains an address in memory.  Pointers were first introduced with references which are indicated by the `&` symbol and borrow the value that they point to.  References do not have any overhead.

A smart pointer will act like a normal pointer but contains the ability to store metadata.  One example of a smart pointer is a reference counter which keeps track of the number of owners.  Using a reference pointer allows for multiple owners of single data.  After all owners have been cleaned up, it is possible to cleaup the smart pointer itself.

Another important difference between references (a type of pointer) and other types of smart pointers is that references only borrow data while many smart pointers allow for owning of data.

Two basic types of smart pointers are `String` and `Vec<T>`.  These are considered smart pointers as they both own some memory which can be manipulated.

Typically, smart pointers are implemented with `structs` and will have `deref` and `drop` traits.  The `deref` trait allows for the smart pointer to be referenced while the `drop` traits behaves similarly to a C++ deconstructor.

The most common standard library pointers are:
* Box<T>: used for allocating  values on the heap
* RC<T>: a reference counting type which allows for multiple owners
* Ref<T> and RefMut<T>: accessed through RefCall<T>, a type that enforces borrowing rules at runtime rather than at compiletime.

#### Using Box<T> to Point to Data on the Heap
A box allows for the storage of data on the heap rather than on the stack.  The stack will still hold a pointer to the heap data.  Boxes do not have performance overhead although they are limited in their capabilities.  They are most often used in these situations:
* When a type's size is unknown at compile time and you want to use a value of that type in a context that requires an exact size
* When you there is a large amount of data that you want to transfer ownership of and want to ensure that the data will not be copied
* When you want to own a value and only care about it being a type that implements a particular trait rather than being of a specific type

##### Enabling Recursive Types with Boxes
At compile time, Rust needs to know how much space a type takes up.  Once type whose size can't be known at compile time is a recursive type, where a value can have as part of itself, another value of the same type.  As this nesting could be infinite, Rust does not know how much space a value of a recursive type needs.  Boxes however, have a known size so by inserting a box in a recursive type definition, recursive types are resolved!

###### More Information About the Cons List
A _cons list_ is a data structure that comes from Lisp and its dialects.  In Lisp, the `cons` function constructs a new pair from its two arguments which are usually a single value and another pair.  A common idiom: "to cons x into y" informally means that a new container will be constructed with x at the head followed by the container y.

###### Computing the Size of a Non-Recursive Type
The greatest possible size of an object is used when allocating memory for it.  For example, in an `enum`, the largest variant will decide the total size of the `enum`.

###### Using `Box<T>` to Get a Recursive Type with a Known Size
Because a `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` needs (a pointer's size does not depend on how much data it is pointing to!).  In this case, a recursive data type can store a pointer to another version of itself.

#### Treating Smart Pointers Like Regular References with the `Deref` Trait
Implementing the `deref` trait allows for users to customize the behavior of the dereference operator, `*`.  By Implementing `deref` in such a way that a smart pointer be treated like a regular reference, code can be written which operates on references and use that code with smart pointers as well.

##### Treating a Type Like a Reference by Implementing the `Deref` Trait
Begin by `use`ing `std::ops::Deref`.  Then, in the implementation for `Deref`, define an associated type for the `Deref` trait to use; this can be done using `type Target = T` where `T` is the generic parameter.  Associated types are a slightly different way of declaring a generic parameter and they will be covered in more detail later on.

The actual `fn deref` should return the 0th index of `self` which looks like:
```
fn deref(&self) -> &T {
	&self.0
}
```
when we dereference the result of this `deref` function, Rust behind the scenes will convert `*y`, for example, into `*(y.deref())`.

Also note that `deref` returns a reference to the `self.0` so that we are not moving ownership out of `self`.

##### Implicit Deref Coercions with Functions and Methods
_Deref Coercion_ is a convenience Rust performs on arguments to functions and methods.  Deref coercion will convert a reference to a type that implements `deref` into a reference to a type that `deref` can convert the original type into.  Deref coercion happens automatically when we pass a reference to a particular type's value as an argument to a function or method that doesn't match the parameter type in the function or method definition.  A sequence of calls to the `deref` method converts the type we provided into the type the parameter needs.  An example in action; consider the following function:
```
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
```
This `hello` function will accept a string slice as an argument.  However, deref coercion allows us to call `hello` with a reference to a value of type `MyBox<String>`.

##### How Deref Coercion Interacts with Mutability
Similar to how the `deref` trait can be used to override the `*` operator on immutable references, the `DerefMut` trait can be used to override the `*` operator on mutable references.

Rust will use deref coercion when it finds types and trait implementations in three cases:
* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`

The first two cases are the same except that the second one involves mutability.  The first case states that if you have `&T`, and `T` implements `deref` to some type `U`, you can get `&U` transparently.

The third case is trickier.  Rust can coerce a mutable type into an immutable one!  **The reverse however is not possible!**

#### Running Code on Cleanup with the `Drop` Trait
`Drop` is another important trait for smart pointers.  `Drop` allows a user to specify the behavior for a type whenever a value of that type is about to go out of scope.

It is also possible to invoke an early `drop` using `std::mem::drop` which is different from simply invoking `value.drop()`.  `sed::mem::drop` can be used as follows: `drop(value)`.  Note that if there is an attempt to drop a value before it is out of use, Rust will not actually follow through with the drop.

#### `Rc<T>`, the Reference Counted Smart Pointer
In the majority of cases, ownership is clear; this, however, is not always the case as there are times when a single value may want to have multiple owners.  In a graph data structure, for example, multiple edges might point to the same node and that node is conceptually owned by all of the edges that point to it.

The `Rc<T>` type can be used to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish data last.  If we knew which part would finish last, we could just make that part the data's owner, and the normal ownership rules enforced at compiler time would take effect.

When using an `Rc<T>`, the `Rc::clone` is often used which is different from the typical `clone` in that `clone` makes a deep copy while `Rc::clone` increments the counter.  The reference counter of a smart pointer can be accessed with `Rc::strong_counter(&Rc)`.

#### `RefCell<T>` and the Interior Mutability Pattern
`Interior mutability` is a design pattern in Rust that allows one to mutate data even when there are immutable references to that data.  The `RefCell<T>` type follows this inner mutability pattern.

##### Enforcing Borrowing Rules at Runtime with `RefCell<T>`
Unlike `Rc<T>`, `RecCell<T>` represents single ownership over the data it holds.  What makes `RefCell<T>` from a type like `Box<T>`?  Recall the following borrowing rules:
* At any given time, you can have _either_ (but not both of) one mutable reference or any number of immutable references
* References must always be valid

With references and `Box<T>`, the borrowing rules' invariants are enforced at compile time.  With `RefCell<T>`, these invariants are enforced at _runtime_.  This will manifest in the form of a `panic!` call which will exit a program.  The `RefCell<T>` type is useful when one can be sure that code will follow all borrowing rules however the compiler is unable to understand and guarantee that.

##### Keeping Track of Borrows at Runtime with `RefCell<T>`
When creating immutable and mutable references, the `&` and `&mut` syntax is used respectively.  With `RefCell<T>`, the `borrow` and `borrow_mut` are used instead.  Every time `borrow` is called a smart pointer of type `Ref<T>` is returned whereas `borrow_mut` returns `RefMut<T>`; both types implement `deref` and can be treated as regular references.

The `RefCell<T>` keeps track of how many times `Ref<T>` and `RefMut<T>` smart pointers are currently active.  Every time `borrow` is called, the `RefCall<T>` increases its count of how many immutable borrows are active.  When a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by one.

##### Preventing Reference Cycles: Turning an `Rc<T>` into a `Weak<T>`
A reference cycle can occur when one reference refers to another and that reference refers back to the original.  One way to prevent reference cycles is to use `Weak<T>` smart pointers.  A `weak` type smart pointer will increment/decrement the `weak_counter`.  A `weak_counter` is different from a `smart_counter` in that the `weak_count` does not have to be 0 for the `Rc<T>` instance to be cleaned up.  In order to ensure that a value has not been dropped, use the `upgrade` method of a `weak` type to return an `Option<T>` which will be `Some` if the `Rc<T>` value has not been dropped and `None` if the `Rc<T>` value has been dropped.

One example in which a `weak` type can be used is when creating a tree data structure.  A parent should be aware of its children and vice-versa; when a parent is dropped, its parents should be as well, however the same is _not_ true the other way around.

### Chapter 16
_Concurrent programming_ is programming in which different parts of a program execute independently where _parallel programming_ is programming in which different parts of a program execute at the same time.  The Rust ownership system is used to manage memory safety and concurrency problems.

#### Using Threads to Run Code Simultaneously
In most current operating systems, an executed program's code is run in a process, and the operating system manages multiple processes at once.  Within a program, there can also be independent parts that run simultaneously.  The features that run these independent parts are called threads.

There are two main ways in which programming languages implement threads.  Many operating systems provides API's for creating new threads.  This model in which a language depends on an OS for the creation of new threads is sometimes called _1:1_ meaning one operating system thread per one language thread.

The second model involves programming languages providing their own special implementation of threads.  Programming language-provided threads are called _green_ threads and languages which use these green threads will execute them in the context of a different number of operating system threads.  For this reason, the green-threaded model is called the M:N model as there are `M` green threads per `N` operating system threads where `M` and `N` need not be the same number.

Each model has its own advantages and trade-offs, and the trade-off most important to Rust is runtime support.  _Runtime_, in this context, refers to code that is included by the language in every binary.  Rust strives to have an extremely small runtime.  The green threading M:N model requires a larger language runtime to manage threads.  As such, the Rust standard library only provides an implementation of 1:1 threading.  There are, however, crates which provide M:N threading.

##### Creating a New Thread with `spawn`
To create a new thread, call the `thread::spawn` function and pass a closure containing the code to be run in the new thread.  A thread will be stopped when the outer context finishes (eg: a thread run in main will terminate when main ends)

##### Waiting for All Threads to Finish Using `join` Handles
In order to pause surrounding execution until a thread has finished executing, save the value of `thread::spawn` into a variable.  The return type for `thread::spawn` is a `JoinHandle`.  A `JoinHandle` is an owned value that, when we call the `join` method on it, will wait for its thread to finish.  Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates.  _Blocking_ a thread means that the thread is prevented from performing work or exiting.

##### Using `move` Closures with Threads
The `move` closure is often used alongside `thread::spawn` because it allows the use of data between threads.  Rust will infer how to capture an 'environment' variable and as it is very difficult to tell when a thread will end it is hard to know when a reference will no longer be valid.

#### Using Message Passing to Transfer Data Between Threads
An increasingly popular approach to ensuring safe concurrency is _message passing_ in which threads or **actors** communicate by sending messages to each other which contain data.  *"Do not communicate by sharing memory; instead, share memory by communicating."* -GoLang Docs

Rust standard library comes with a standard library implementation of channels called _channels_.  A channel in programming has two halves: a transmitter and a receiver.  The transmitter half is the upstream location and the receiver half is where the downstream location is.  A channel is said to be closed if either the transmitter or receiver half is dropped.

Channels in Rust are created using the `mpsc::channel` function which returns a tuple.  `mpsc` stands for _multiple producer, single consumer_.  The tuple is typically written as `(tx, rx)` which stand for transmitter and receiver.  The transmitter end can be moved into the spawned thread and have it send one string to the main thread to communicate.

The transmitting end has a `send` method that takes the value to be sent.  The `send` method returns a `Result<T, E>` type so that if the receiving end has already been dropped and there is nowhere to send the value, the send operation will return an error.

The receiving end of a channel has two useful methods: `recv` and `try_recv`.  The `recv` call will block the caller's thread execution until a value is sent down the channel.  The `try_recv` method will not block but will instead `Result<T, E>` immediately: an `Ok` value holding a message if one is available and an `Err` value if there aren't any messages at the time.  `try_recv` is useful if the calling thread has work to do while waiting for messages.  A loop could be written that calls `try_recv` every so often, handles a message if one is available, and otherwise does other work for a while until checking again.

##### Channels and Ownership Transference
After sending a value down a channel, it cannot be used again as ownership has been transferred.

##### Sending Multiple Values and Seeing the Receiver Waiting
The `rx` part of the tuple received from `mpsc::channel` can be treated as an iterator to iterate over all messages received as they are received.  When the channel is closed, the iteration will terminate.

##### Creating Multiple Producers by Cloning the Transmitter
As stated before, `mpsc` stands for _multiple producer, single consumer_ meaning we can create multiple threads which all send values to the same receiving thread.  This requires cloning the transmitting half of the channel.

#### Shared-State Concurrency
Message passing is similar to single ownership in that once a value has been transferred down a channel, it can no longer be used in the original context.  Shared memory concurrency is like multiple ownership in that multiple threads can access the same memory location at the same time.

##### Using Mutexes to Allow Access to Data from One Thread at a Time
_Mutex_ is an abbreviation for _mutual exclusion_ meaning a mutex allows for only one thread to access some data at a particular time.  To access the data in a mutex, a thread must first signal that it wants to access by asking to acquire the mutex's _lock_.  The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.  The mutex serves to _guard_ the data it holds using the locking system.

When using mutexes, one must remember two rules:
* You must attempt to acquire the lock before using the data
* When done with the data being guarded by the muetx, you must unlock the data so other threads can acquire the lock

##### The API of `Mutex<T>`
Mutex's can be found within the `std::sync::Mutex` library and a new mutex can be created using the `new` function.  To access the data inside of a mutex, the `lock` method is used to acquire the lock.  This call will block the current thread so that it cannot do any work until it's our turn to have the lock.  The call to `lock` would fail if another thread holding the lock panicked.  In that case, no one would ever be able to get the lock, so we've chosen to `unwrap` and have this thread panic if we're in that situation.  As a reminder, `unwrap` will implicitly return the inner element or `panic` - we check the `Option<T>` enum `Some(T)` or `None` enumerations.

After a lock has been acquired, the return value can be treated as a mutable reference to the data inside.  Rust's type system ensures that a lock is acquired before the value inside is used.

`Mutex<T>` is a smart pointer.  More accurately, the call to `lock` returns a smart pointer called `MutexGuard`.  This smart pointer implements `deref` to point at the inner data.  The smart point also has a `drop` implementation that releases the lock automatically when a `MutexGuard` goes out of scope; this prevents forgetting to release a `lock` after acquiring it.

##### Sharing a `Mutex<T>` Between Multiple Threads
Rust does not allow for multiple thread's to take ownership of the same value in the same context.  For example, a loop generating n > 1 threads cannot use `move` to take ownership of environment variables.

##### Multiple Ownership with Multiple Threads
One initial attempt to solve the 'sharing between `Mutex<T>`s problem could be to wrap the environment variable we want to use in an `Rc<T>` and cloning before transferring ownership into a new thread.  This however, does not quite work.  This is because `Rc<T>` is not safe to share across threads!

`Rc<T>` manages reference counts by incrementing for each call to `clone` and decrementing each time a clone is dropped.  It does not however adhere to concurrent primitives to make sure changes to count can't be interrupted by another thread.

##### Atomic Reference Counting with `Arc<T>`
There is a type, `Arc<T>` which is like `Rc<T>` that is safe to use in concurrent situations.  The `A` here stands for atomic meaning it's an atomically reference counted type.  Atomic primitive types come with some overhead which is why all primitives are not atomic.  One nice thing about `Arc<T>` is that it has the same API as `Rc<T>` meaning there is no need to change and usage.

#### Extensible Concurrency with `Sync` and `Send` Traits
Rust the language has very few concurrency features.  Almost every concurrency feature covered so far is a part of the standard library and not the language itself.  There are, however, two concurrency concepts embedded in the language: `std::marker` traits `sync` and `send`.

##### Allowing Transference of Ownership Between Threads with `Send`
The `send` marker trait indicates that ownership of the type implementing `send` can be transferred between threads.  Almost every Rust type is `send` but there are some exceptions.  One exception we have covered is `Rc<T>` which cannot guarantee concurrent safety with `clone`.

Any type which is composed entirely of `send` types is automatically marked as `send` as well.  Almost all primitive type are `send`, aside from raw pointers.

##### Allowing Access from Multiple Threads with `sync`
The `sync` marker trait indicates that it is safe for the type implementing `sync` to be referenced from multiple threads.  In other words, any type `T` is `sync` if `&T` is `send`.  Similar to `send`, primitive types are `sync`, and types composed entirely of `sync` are also `sync`.

The smart pointer `Rc<T>` is also not `sync` for the same reasons that it is not `send`.  The `RefCell<T>` type and the family of related `Cell<T>` types are not `sync`.  The implementation of borrow checking that `RefCell<T>` does at runtime is not thread-safe.

##### Implementing `send` and `sync` Manually is Unsafe
As types that are entirely made up of `send` and `sync` traits are automatically also `send` and `sync`, these traits never have to be implemented manually.  As marker traits, they don't even have any methods to implement manually.

### Chapter 17
#### Characteristics of Object-Oriented Languages
There is no consensus in the programming community about what features a language must have in order to be considered object-oriented.  Some commonly discussed characteristics are: objects, encapsulation and inheritance.

##### Objects Contain Data and Behavior
"Object-oriented" programs are made up of objects.  An _object_ packages both data and procedures that operate on that data.  The procedures are typically called _methods_ or _operations_."

Using the above definition, Rust is object oriented.  Structs and enums have data, and `impl` blocks provide methods on structs and enums.  Even though structs and enums are not called objects, they provide the same functionality, according to the above definition, as objects.

##### Encapsulation that Hides Implementation Details
Another aspect commonly associated with OOP is the idea of _encapsulation_, which means that the implementation details of an object aren't accessible to code using that object.  Therefore, the only way to interact with an object is through its public API; code using the object shouldn't be able to reach into the object's internals and change data or behavior directly.

Encapsulation in Rust is controlled using keywords like `pub` which decide which modules, types, functions and methods in a program should be public.  By default, all code is private.

##### Inheritance as a Type System and as Code Sharing
_Inheritance_ is a mechanism whereby an object can inherit from another object's definition, thus gaining the parent object's data and behavior without having to define them again.

In Rust, there is no way to define a struct which inherits a parent struct's fields and method implementations.  There are, however, work arounds for implementing inheritance-like behavior in Rust.

Inheritance is a desirable feature for two main reasons.  The first is that inheritance allows for more reuse of code.  Rust code can be shared using default trait method implementations instead.  Any type implementing a particular trait has access to that trait's methods.

The second reason is polymorphism.  Polymorphism is often times thought of as being the same thing as inheritance; this is not the case.  Polymorphism is actually more general than inheritance and allows for code to work with data of multiple types.  For inheritance, these types are typically sub-classes.  Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide (similar to Java's interfaces?)

Recently, inheritance has fallen out of favor as a programming design solution as it typically overshares and can define behavior for a parent class that is not actually applicable to a child class.  Rust takes a different approach, using trait objects instead of inheritance.

#### Using Trait Objects that Allow for Values of Different Types
An early problem of vectors in Rust was that they only allowed for the storage of elements of one type.  It is possible to create a workaround which uses `enums` to define a finite set of possible types within a vector.  The issue with this approach is that it requires the knowledge of all possible types at compiletime which may not be possible.

##### Defining a Trait for Common Behavior
A trait object points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime.  A trait object can be used in place of a generic or concrete type.  Wherever there is a trait object, Rust's type system will ensure at compile time that any value used in that context will implement the trait object's trait.

In Rust, structs and enums are not referred to as objects..  In a struct or enum, the data in the struct field and the behavior in the `impl` blocks are separated.  A _trait object_ is more similar to an object in other languages as data and behavior are combined.  A trait object is different from an object in other languages as it is not possible to add data to a trait object.  Trait objects are not as helpful as objects in other languages as their specific purpose is to allow abstraction across common behavior.

A trait can be defined as:
```
pub trait Draw {
	fn draw(&self);
}
```
A trait can then be used with a trait object as:
```
pub struct Screen {
	pub components: Vec<Box<dyn, Draw>>,
}
```
The above syntax defined a struct named `Screen` which holds a vector named `components`.  This vector is of type `Box<dyn Draw>` which is a trait object; it's a stand-in for any type inside a `Box` that implements the `Draw` trait.

On the `Screen` struct, we define a method named `run` that will call the `draw` method on each of its `components`:
```
impl Screen {
	pub fn run(&self) {
		for component in self.components.iter() {
			component.draw();
		}
	}
}
```
The advantage of using trait objects and Rust's type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn't implement a method but we call it anyway.

Recall that _static dispatch_ is when the compiler knows methods being called at compile time and can substitute for generics.  _Dynamic dispatch_, on the other hand, is when the compiler can't tell what method's being called at compile time and the compiler emits code that runs at runtime which will be used to figure out the call.

When using trait objects, Rust **must** use dynamic dispatch.  The compiler doesn't know all the types that might be used with the code that is using trait objects, so it doesn't know which method implemented on which type to call.  Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call.  **There is no runtime cost** when this lookup happens **that doesn't occur with static dispatch**.  Dynamic dispatch also prevents the compiler from choosing to inline a method's code, which in turn prevents some optimizations.  On the other hand, we did get extra flexibility in the code so it's a trade-off to consider.

##### Object Safety is Required for Trait Objects
You can only make _object-safe_ traits into trait objects.  Some complex rules govern all the properties that make a trait object safe, but in practice, only two rules really are relevant:
* The return type **isn't** self
* There are no generic type parameters

The `self` keyword is an alias for the type we're implementing the traits or methods on.  Trait objects must be object safe because once they've been used, Rust no longer knows the concrete type that's implementing that trait.  If a trait method returns a concrete `self` type, but a trait object forgets the exact type that `self` is, there is no way the method can use the original concrete type.  The same is true for generic type parameters that are filled with concrete type parameters when the trait is used: the concrete types become part of the type that implements the trait.  When the type is forgotten through the use of a trait object, there is no way to know what types to fill in the generic type parameters with.

#### Implementing an Object-Oriented Design Pattern
The _state pattern_ is an object-oriented design pattern.  In a nutshell, the pattern is that a value has some internal state which is represented by a set of _state objects_.  The values behavior will depend on the internal state.

Using the state pattern means that when the business requirements of the program change, we won't need to change the code of the value holding the state or the code that uses the value.  We only need to update code inside one of the state objects to change its rules or perhaps add more state objects.

### Chapter 18
Patterns in Rust allow for matching against the structure of types, both complex and simple.  Patterns can be used in conjunction with `match` expressions.  A pattern consists of some of the combination of the following:
* Literals
* Destructured arrays, enums, structs, or tuples
* Variables
* Wildcards
* Placeholders

These components describe the shape of the data we're working with, which we then match against values to determine whether our program has the correct data to continue running a particular piece of code.

#### All the Places Patterns Can Be Used
##### `match` Arms
One place in which patterns are used is in the arms of `match` expressions.  Formally, `match` expressions are defined as the keyword `match`, a value to match on, and one or more match arms that consist of a pattern and an expression to run if the value matches that arm's pattern.  E.g.:
```
match VALUE {
	PATTERN => EXPRESSION,
	PATTERN => EXPRESSION,
	PATTERN => EXPRESSION,
}
```
One requirement for the `match` expressions is that they need to be exhaustive in the sense that all possibilities for the values in the `match` expression must be accounted for.  One way to ensure you've covered all possible patterns is to have a catchall pattern for the last arm.

A particular pattern, `_` will match anything, but it never binds to a variable, so it's used often in the last match arm.  The `_` pattern can be useful when you want to ignore any value not specified.

##### Conditional `if let` Expressions
`if let` was previously discussed as a shorter way of writing the equivalent `match` that only matches a single case.

##### `while let` Conditional Loops
The `while let` conditional loop allows a `while` loop to run for as long as a pattern continues to match.

##### `for` Loops
As was mentioned before, the `for` loop is the most common loop construction in Rust code.  In a `for` loop, the pattern is the value that directly follows the keyword `for`, so in the `for x in y` the `x` is the pattern.

##### `let` Statements
`let` itself takes a pattern.  Consider `let PATTERN = EXPRESSION`.  In statements like `let x = 5;` with a variable name in the `PATTERN` slot, the variable name is just a particularly simple form of a pattern.  Rust compares the expression against the pattern and assigns any names it finds.  So in the `let x = 5;` example, `x` is a pattern that means "bind what matches here to the variable `x`."  Because the name `x` is the whole pattern, this pattern effectively means "bind everything to the variable `x`, whatever the value is."

A more complex example using `let` is: `let (x, y, z) = (1, 2, 3);`

##### Function Parameters
Function parameters can also be patterns.  The parameter in a function declaration is actually specifying a pattern to take in.  This becomes more evident in examples like:
```
fn print_coordinates(&(x, y): &(i32, i32)) {
	println!("Current location: ({}, {})", x, y);
}

fn main() {
	let point = (3, 5);
	print_coordinates(&point);
}
```

#### Refutability: Whether a Pattern Might Fail to Match
Patterns come in two forms: refutable and irrefutable.  Patterns that will match for any possible value passed are _irrefutable_.  An example would be `x` in the statement: `let x = 5;` because `x` matches anything and therefore cannot fail to match.  Patterns that can fail to match for some possible value are _refutable_.  An example would be `Some(x)` in the expression `if let Some(x) = a_value` because if the value in the `a_value` variables is `None` rather than `Some`, the `Some(x)` pattern will not match.

Function parameters, `let` statements, and `for` loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don't match.  The `if let` and `while let` expressions only accept refutable patterns, because by definition they're intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure.

#### Pattern Syntax
##### Matching Literals
As was seen previously, it is possible to match patterns against literals directly:
```
let x = 1;

match x {
	1 => println!("one!"),
	2 => println!("two!"),
	3 => println!("three!"),
	_ => println!("anything!"),
}
```

##### Matching Named Variables
Named variables are irrefutable patterns that match any value.  There is one complication when using a named variable inside of a `match` expression.  Because `match` starts a new scope, variables declared as part of this pattern inside the `match` expression will shadow those with the same name outside of the `match` construct as is the case with all variables.

Take for example the following code:
```
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
```
If we walk through the different outcomes, we see that initially `Some(5) != Some(50)` but the second case will match as `Some(y)` is referring to a new `y` that is new to this match scope and the new `y` will match any value inside of a `Some` which is what we have in `x`.  Therefore, with the current setup of the code, the output from the match will be `Matched, y = 5`.

If we changed the example to be:
```
fn main() {
    let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

```
we are not considering a different case in which the first two arms of the`match` are not matched but the third is.  This will cause the `Match` to print: `Default case, x = None`.

From this example, we can see how it is possible to overshadow values within a match to produce different behavior.

##### Multiple Patterns
In `match` expressions, it is possible to match multiple patterns using the `|` syntax which means _or_.

##### Matching Ranges of Values with `...`
The `...` syntax allows us to match an inclusive range of values.  Ranges are only allowed with numeric values or `char` values because the compiler needs to be able to check that the range isn't empty at compile time.  The only types for which Rust can tell if a range is empty or not are `char` and numeric values.

##### Destructuring to Break Apart Values
Patterns can also destructure structs, enums, tuples and references to use different parts of these values.

###### Destructuring Structs
A struct is constructed with fields.  These fields can be pulled out of the struct using a `let` statement which can either use names to refer to fields, or use the literal names for each field.

###### Destructuring Enums
When destructuring enums, make sure to destructure using the same pattern that the enum was defined with.

###### Destructuring Nested Structs and Enums
Similar to destructuring a singly nested enum, first match the outer enum and then the inner enum.

###### Destructuring Structs and Tuples
Complex types can be broken down into their component parts so that the values can be used separately.

##### Ignoring Values in a Pattern
It can be helpful to use `_` or `..` to ignore remaining parts of a value.

###### Ignoring an Entire Value with `_`
The `_` value can be used as a wildcard pattern that will match any value but not bind the value.  It is often used as the last arm in a `match` expression but can also be used in any pattern, including function parameters.

###### Ignoring Parts of a Value with a Nested `_`
The `_` can also be used inside of another pattern to ignore just part of the value.  For example, when it is desirable to test for only part of a value but have no use for the other parts in the corresponding code we want to run.

###### Ignoring an Unused Variable by Starting its Name with `_`
If a variable is created which is not used anywhere, Rust will warn that its name should be changed to include a `_` in front of it.  This is because you can tell Rust not to warn you about an unused variable by starting the name of the variable with an underscore.

###### Ignoring Remaining Parts of a Value with `..`
With values that have many parts, the `..` syntax can be used to ignore the majority of these parts.  The `..` pattern ignores any parts of a value that haven't been explicitly matched in the rest of the pattern.  For example:
```
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```
This is a faster way of ignoring `y` and `z` than having included `y` and `z` as: `y: _` or `z: _`.

##### Extra Conditionals with Match Guards
A _match guard_ is an additional `if` condition specified after the pattern in a `match` arm that must also match, along with the pattern matching, for that arm to be chosen.  Match guards are useful for expressing more complex ideas than a pattern alone allows.

##### `@` Bindings
The _at_ operator allows for the creation of a variable that holds a value at the same time that value is being tested to see if it matches a pattern.

### Chapter 19
#### Unsafe Rust
The majority of common Rust code is written uses Rust's memory safety guarantees enforced at compile time.  Rust, however, has a 'secret second language' hidden inside it that doesn't enforce these memory safety guarantees called _unsafe Rust_. Unsafe Rust works just as regular Rust does, however it allows for some additional superpowers.

Unsafe exists for the reason that, by nature, static analysis is conservative.  When a compiler tries to determine whether or not code upholds guarantees, it's better for it to reject some valid programs rather than accept some invalid programs.

To begin using unsafe Rust, surround a new block with the keyword `unsafe`.  This enables four new actions in Rust called _unsafe superpowers_ which include:
* Dereferencing raw pointers
* Calling an unsafe function or method
* Accessing or modifying a mutable static variable
* Implementing an unsafe trait

It's important to understand that unsafe Rust does not turn of the borrow checker or disable any other of Rust's safety checks.  If unsafe code is used, that unsafe is still checked.  The `unsafe` keyword only allows for access to these four features that are then not checked by the compiler for memory safety.

To isolate unsafe code as much as possible, it is best to enclose unsafe code within a safe abstraction provided by a safe API.

##### Dereferencing a Raw Pointer
As with references, raw pointers can be immutable or mutable and are written as `*const T` and `*mut T` respectively.  Note that the **asterisk is no the dereference operator; it's part of the type name**.  In the context of raw pointers, _immutability_ means that the pointer can't be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:
* Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple pointers to the same location
* Aren't guaranteed to point to valid memory
* Are allowed to be null
* Don't implement any automatic cleanup

A raw pointer can be created in safe code, they just cannot be dereferenced outside an unsafe block.

##### Calling an Unsafe Function or Method
###### Creating a Safe Abstraction over Unsafe Code
Just because a function contains unsafe code doesn't mean we need to mark the entire function as unsafe.  In fact, wrapping unsafe code in a safe function is a common abstraction.

One example of an unsafe function that can be wrapped in a safe function is the standard `split_at_mut` function.  A naive implementation might look like:
```
fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid],
     &mut slice[mid..])
}
```
When this code tries to compiler, an error is thrown as safe Rust cannot understand that while there are two borrows from the same slice twice, two distinct, non-overlapping parts of the string are being borrowed which is not going to cause an issue.  A better attempt is:
```
fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
    }
}
```
Recall that slices are a pointer to some data and the length of the string.  Here, the `as_mut_ptr` method is used to access the raw pointer of a slice.  As we started off with a mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer with the type `*mut i32` which is stored in the variable `ptr`.

The `slice::from_raw_parts_mut` is takes a raw pointer and a length, and creates a slice.  This function is used to create a slice that starts from m`ptr` and is `mid` items long.  Then `offset` is called on `ptr` with `mid` as an argument to get a raw pointer that starts at `mid` and a slice is created using that pointer and the remaining number of items after `mid` as the length.

The `slice::from_raw_parts_mut` is unsafe because it takes a raw pointer and must trust that this pointer is valid.  The `offset` method on raw pointers is also unsafe because it must trust that the offset location is also a valid pointer.

##### Using `extern` Functions to Call External Code
Sometimes, Rust code might need to interact with code written in another language.  For this, the `extern` keyword should be used.  `extern` will facilitate the use of a _Foreign Function Interface (FFI)_.  Functions called with an `extern` block are always unsafe to call from Rust code as Rust cannot guarantee their safety.

Within an `extern` block, the names and signatures of external functions from other languages are listed.  For example, to work with the `C` language standard library use:
```
extern "C" {
	... C code ...
}
```
The `"C"` part of `extern "C"` defines which _application binary interface (ABI)_ the external function uses.

It is also possible to define functions which can be called from other languages.  To do this, rewrite a function header to look like:
```
#[no_mangle]
<visibility> extern "<Target_Language>" fn <fn_name> {... Rust code ...}
```
The `#[no_mangle]` in this case will ensure that the Rust compiler does not 'mangle' the function name which is done by most compilers in most languages to make function names more useful for compilers.


##### Accessing or Modifying a Mutable Static Variable
Rust does support global variables although they can be problematic with Rust's ownership rules.  If two threads are accessing the same mutable global variable, for example, this could lead to a **data race**.

In Rust, global variables are called _static_ variables.  Static variables are similar to constants.  The names of static variables should be in _screaming snake case_ and the variable's type must be annotated.  Static variables can only store references with `'static` lifetime which means the Rust compiler can figure out the lifetime and it does not need to be annotated explicitly.  Accessing an immutable static variable is safe.

Constants and immutable static variables may seem similar but there is a subtle difference.  A static variable will **always** have a fixed address in memory; using the value will always access the same data.  Constants, on the other hand, are allowed to duplicate their data whenever they're used.

Another difference between constants and static variables is that static variables can be mutable.  Accessing and modifying mutable static variables is _unsafe_.

##### Implementing an Unsafe Trait
The final action that works only with `unsafe` is implementing unsafe traits.  A trait is considered to be unsafe when at least one of its methods has some invariant that the compiler can't verify.  We can declare a trait as unsafe by adding the `unsafe` keyword before `trait` and marking the implementation of the trait as `unsafe` too.

#### Advanced Lifetimes
So far, lifetime annotations have been discussed in the context of providing explicit lifetimes to denote how long a reference should be valid.  Most of the time, however, Rust allows us to elide these explicit lifetime annotations.  There are three advanced features of lifetimes left to cover:
* Lifetime subtyping: ensures that one lifetime outlives another lifetime
* Lifetime bounds: specifies a lifetime for a reference to a generic type
* Inference of trait object lifetimes: allows the compiler to infer trait object lifetimes and when they need to be specified

##### Ensuring One Lifetime Outlives Another with Lifetime Subtyping
_Lifetime Subtyping_ specifies that one lifetime should outlive another lifetime.  Lifetime subtyping can be done by first declaring a single lifetime and then declaring another lifetime which lives at least as long as this first one.  For example:
```
struct Parser<'c, 's: 'c> {
	context: &'c Context<'s>,
}
```
Here, we ensure that the lifetime of `'s` lasts at least as long as the lifetime of `'c`.

##### Lifetime Bounds on References to Generic Types
Lifetime parameters can be added as constraints on generic types which are called _lifetime bounds_.  Lifetime bounds help Rust to verify that references in generic types won't outlive the data they're referencing.  For example, consider the definition of the `Ref` struct:
```
struct Ref<'a, T>(&'a T);
```
Here, the issue is that `T` can be any type, `T` could be a reference or a type that holds one or more references, each of which have their own lifetimes.  Rust cannot be sure that`T` will live as long as `'a`.

##### Inference of Trait Object Lifetimes
What should happen if the type implementing the trait in the trait object has a lifetime of its own.  Rust comes with some rules for working with lifetimes and trait objects:
* The default lifetime of a trait object is `'static`
* With `&'a Trait` or `&'a mut Trait`, the default lifetime of the trait object is `'a`
* With a single `T: 'a` clause, the default lifetime of the trait object is `'a`
* With multiple clauses like `T: 'a`, there is no default lifetime; we must be explicit

In order to be explicit when there are multiple clauses, a lifetime bound on an object can be added with syntax similar to: `Box<dyn Red + 'a>`

#### Advanced Traits
##### Specifying Placeholder Types in Trait Definitions with Associated Types
_Associated types_ connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.  The implementor trait will specify the concrete type to be used in this type's place for the particular implementation.  In this way, a trait can be defined that uses some types without needing to know exactly what those types are until the trait is implemented.  An example of an associated type in action:
```
pub trait Iterator {
	type Item;
	
	fn next(&mut self) -> Option<Self::Item>;
}
```
The type `Item` is a placeholder type and the `next` method's definition shows that it will return values of type `Option<Self::Item>`.  Implementors of the `Iterator` trait will specify the concrete type for `Item`, and the `next` method will return an `Option` containing a value of that concrete type.

Associated types may similar to generics however they are different concepts.  The difference is that when using generics, the types in each implementation must be implemented as generics allow for uses like: `Iterator<String> for Counter` and `Iterator<u32> for Counter`.  When a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time.

With associated types, it is not necessary to annotate types because it is not possible to implement a trait on a type multiple times.  In the example above, the type of `Item` can only be chosen _once_!  For this reason, it is not necessary to specify that the iterator is for values of `u32` whenever `next` is used.

##### Default Generic Type Parameters and Operator Overloading
When using generic type parameters, default concrete types can be specified for the generic type.  This eliminates the need for implementors of the trait to specify a concrete type if the default type works.  The syntax for specifying a default type for a generic type is: `<PlaceholderType=ConcreteType>` when declaring the generic type.

An example where this comes in handy is when using _operator overloading_.  Operator overloading is customizing behavior of an operator (such as `+`) in particular situations.

Rust does not allow for the creation of new operators or overloading of arbitrary operators but it is possible to overload operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator.  For example, to overload the `+` operator to add two `Point` instances together.

##### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
Nothing in Rust prevents a trait from having a method with the same name as another trait's method, nor does Rust prevent implementing both traits on one type.  It is also possible to implement a method directly on the type with the same name as methods from traits.

When calling methods with the same name, it is important to Rust to specify which one to use.  Consider the case where we define two traits that both implement the same method.

Specifying the trait name before the method name clarifies to Rust which implementation of a method is being called.  This only works when the method being called takes in a reference to `self` as a parameter.  Associated functions, however, that are not part of traits do not have a `self` parameter.  When two types in the same scope implement that trait, Rust can't figure out which type is being referred to unless _fully qualified syntax_ is being used.

Fully qualified syntax will provide Rust with a type annotation within angle brackets which indicates how to treat a type as a trait in this context.  In general, the full syntax for fully qualified syntax is:
```
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```
For associated functions, there would be no need for a `receiver`: there would only be a need for a list of other arguments.

##### Using Supertraits to Require One Trait's Functionality Within Another Trait
Sometimes, it is desirable to have one trait use another trait's functionality.  In this case, one must rely on the dependent trait being implemented.  This trait, being relied on, is called a _supertrait_ of the trait being implemented.  To specify supertrait dependence, define a trait in the following way:
```
trait <name>: <path_to_supertrait::supertrait> { ... }
```

##### Using the Newtype Pattern to Implement External Traits on External Types
When discussing implementing traits on a type, it was mentioned that we are only allowed to implement traits on a type as long as either the trait or type are local to our crate.  It is possible to get around this restriction using the _newtype pattern_ which involves creating a new type in a tuple struct.  The tuple struct will have one field and be a thin wrapper around the type that we want to implement a trait for.  Then, the wrapper type is local to our crate and we can implement the trait on the wrapper.

For example, we normally are stopped from implementing the `Display` trait on `Vec<T>` which normally is prevented as `Display` and `Vec<T>` are defined outside of our crate.

The example implementation in **newtype_example** uses `self.0` to access the inner `Vec<T>`, because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the tuple.

The downside of using this technique is that the `Wrapper` is a new type so it does not have the methods of the value it's holding.  It would be necessary to implement all of the methods of `Vec<T>` directly on `Wrapper` such that the methods delegate to `self.0` in **newtype_example**.

#### Advanced Types
##### Using the Newtype Pattern for Type Safety and Abstraction
The newtype pattern can be used for things such as enforcing that values are never confused and indicating the units of a value.  Another use of the newtype pattern is in abstracting away some implementing details of a type: the new type can expose a public API that is different from the API of the private inner type if the new type was used directly to restrict the available functionality, for example.

Newtypes can also hide internal implementation.  For example, we could provide a `People` type to wrap a `HashMap<i32, String>` that stores a person's ID associated with their name.  Code using `People` would only interact with the public API we provide, such as a method to add a name string to the `People` collection; that code wouldn't need to know that we assign an `i32` ID to names internally.  The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details.

##### Creating Type Synonyms with Type Aliases
Along with the newtype pattern, Rust provides the ability to declare a _type alias_ to give an existing type another name.  For this, the `type` keyword is used.  For example, we can create the alias `Kilometers` to `i32` like so:
```
type Kilometers = i32
```
Now, the alias `Kilometers` is a _synonym_ for `i32`; unlike `Millimeters` and `Meters` used in a previous example, `Kilometers` is not a separate, new type.  Values that have the type `Kilometers` will be treated the same as values of type `i32`.  As `Kilometers` and `i32` are the same type, they can both be added and `Kilometers` can be passed in place of `i32` parameters.  Using a type alias allows for more manageable code in place of long type annotations.

##### The Never Type that Never Returns
Rust has a special type named `!` that is known as an _empty type_ as it has no values.  In Rust, it is referred to as the _never type_ because it stands in place of the return type when a function will never return.  One use of never types occurs in pattern matching with `match` arms.  Consider the following:
```
let guess = match guess.trim().parse() {
	Ok(_) => 5,
	Err(_) => continue,
}
```
Remember that `match` arms are required to all be of the same type.  As in the `Ok()` case a `u32` is returned, the `Err()` case must also return a type `u32`.  `continue` is an example of a function that returns `!`.  Knowing `continue` returns a never type allows for the `guess` function to return a `u32` type.

To more formally discuss the behavior of the `!` type, `!` can be coerced into any other type.

##### Dynamically Sized Types and the `Sized` Trait
A dynamically sized type is a type whose size can only be determined at runtime.  One common example of a dynamically sized type in Rust is `str`.  Rust needs to know how much memory to allocate for any value of a particular type and all values of a type must use the same amount of memory.  For this reason, if Rust allowed for:
```
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```
both `s1` and `s2` could be of type `str` while `s1` holds 12 bytes and `s2` 15.  To get around this issue, we instead of `str` use `&str`.

Recall that the slice data structure stores the starting position and the length of the slice.  With this data structure, we know the size at compile time (a `str` is twice the size of a `usize`).  The 'golden rule' of dynamically sized types is that we must put them behind some kind of pointer.

Every trait is a dynamically sized type that can be referred to using the name of the trait.  To work with dynamically sized traits, Rust has a particular trait called the `Sized` trait to determine whether or not a type's size is known at compile time.  This trait is automatically implemented for everything whose size is known at compile time.  The trait is automatically implemented for everything whose size is known at compile time.  In addition, Rust implicitly adds a bound on `Sized` to every generic function.  That is, a generic function definition which looks like:
```
fn generic<T>(t: T) { ... }
```
is actually treated as though it had been written like:
```
fn generic<T: Sized>(t: T) { ... }
```
By default, generic functions will only work on types with a known size at compile time.  However, this restriction can be relaxed by defining generic functions like:
```
fn generic<T: ?Sized>(t: &T) { ... }
```
A trait bound on `?Sized` is the opposite of a trait bound on `Sized`: we would read this as "`T` may or may not be `Sized`."  This syntax is only available for `Sized` and not for any other trait.

Also note in the above example that the type of parameter `t` was switched from `T` to `&T`.  Because the type might not be `Sized`, we need to use it behind some kind of pointer.  In this case, we use a reference.

#### Advanced Functions and Closures
##### Function Pointers
So far, passing closures to functions has been covered but it is also possible to pass regular functions to functions.  This technique is useful when it helps to pass a function that has already been defined rather than define a new closure.  Functions coerce to the type `fn` (with a lowercase `f` as `Fn` is separate).  The `fn` type is called a _function pointer_.  The syntax for specifying a parameter which is a function is similar to that of a closure:
```
fn first_func(x: i32) -> i32 {
	x + 1
}

fn second_func(f: fn(i32) -> i32, arg: i32) -> i32 {
	f(arg) + f(arg)
}
```
Unlike closures, `fn` is a type rather than a trait, so `fn` is specified as the parameter type directly rather than declaring a generic type parameter with one of the `Fn` traits as a trait bound.

Function pointers implement all three of the closure traits(`Fn`, `FnMut` and `FnOnce`) so it is always possible to pass a function pointer as an argument for a function that expects a closure.  It's best to write functions using a generic type and one of the closure traits so the function can accept either functions or closures.

##### Returning Closures
Closures are represented by traits which means they can't be returned directly.  In most cases where a trait is trying to be returned, a concrete type that implements the trait can be returned instead.  This cannot be done with closures as they don't have a concrete type that is returnable.

#### Macros
Macros refers to a family of features in Rust:
* _Declarative_ macros with `macro_rules!`
* _Procedural_ macros, which come in three kinds
  * Custom `#[derive]` macros
  * Attribute-like macros
  * Function-like macros

##### The Difference Between Macros and Functions
Fundamentally, macros are a way of writing code that writes other code.  This is known as `metaprogramming`.  The `derive` attribute generates and implementation of various traits.  The `println!` and `vec!` macros are also used frequently.  All of these macros _expand_ to produce more code than the code that has been written manually.

Metaprogramming is useful for reducing the amount of code that is written and maintained, which is also one of the roles of functions.  However, macros have some additional power that functions do not have.

A function signature must declare the number and type of parameters the function has.  Macros, on the other hand, can take a variable number of parameters: we can call `println!("hello")` with one argument or `println!("hello {}", name)` with two arguments. Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type.  A function can't, because it gets called at runtime and a trait needs to be implemented at compile time.

The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because one must write Rust code which can write Rust code.

One last important difference between macros and functions is that macros must be brought into scope _before_ they can be called in a file, whereas functions can be defined anywhere and they can be called anywhere.

##### Declarative Macros with `macros_rules!` for General Metaprogramming
The most widely used form of macros in Rust are _declarative macros_.  These are also sometimes referred to as "macros by example", "`macro_rules!` macros" or just plain "macros".  At their core, declarative macros are written in a form similar to Rust `match `expressions.  As previously discussed, `match` expressions are controls structures that take an expression, compare the resulting value of an expression to patterns, and then run the code associated with the matching pattern.  Macros also compare a value to patterns that have code associated with them; in this situation, the value is the literal Rust source code passed to the macro, the patterns are compared with the structure of that source code, and the code associated with each pattern is the code that replaces the code passed to the macro.  This happens all during compilation.

To define a macro, use the `macro_rules!` construct.  Take a look at a simplified implementation of the `vec!` macro:
```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```
To start, the `#[macro_export]` annotation indicates that this macro should be made available whenever the create in which the macros is defined is brought into scope.  Without this annotation, the macro can't be brought into scope.

The macro definition is started with `macro_rules!` and the name of the macro (defined without `!`).  The `{ }` denote the body of the macro definition.

The structure in the `vec!` body is similar to the structure of the `match` expression.  Here, there is one arm with the pattern `( $( $x:expr ),* )` followed by `=>` and the block of code associated with the pattern.  Anything not matched by a pattern arm will yield an error.

The pattern syntax used for macros is different from the pattern syntax used in other parts of Rust.  First, a set of parentheses encompass the entire pattern.  Then comes a `$` followed by a set of parentheses which captures values that match the pattern within the parentheses to use in the replacement code.  Within `$()` is `$x:expr` which matches any Rust expression and gives the expression the name `$x`.  The comma following `$()` indicates that a literal comma separator character could optionally appear after the code that matches the code captured in `$()`.  The `*` following the comma specifies that the pattern matches 0 or more of whatever precedes the `*`.

Examining the code within the pattern arm, `temp_vec.push()` within the `*()` is generated for each part that matches `$()` in the pattern 0 or more times.  The `$x` is replaced with each expression matched.

##### Procedural Macros for Generating Code from Attributes
The second form of macros is called _procedural macros_ because they are more like functions (which are a type of procedure).  Procedural macros accept some Rust code as an input, operate on that code, and produce some Rust code as an output rather than matching against patterns and replacing the code with other code as declarative macros do.

There are three kinds of procedural macros but they all work in a similar fashion.  First, the definitions must reside in their own crate with a special crate type.  This is for complex technical reasons that will hopefully not stick around for long.

Second, using any of these kinds of macros takes on a form where `some_attribute` is a placeholder for using a specific macro.

##### How to Write a Custom `derive` Macro
For this exercise, refer to the following crates: **/hello_macro**, **/hello_macro/hello_macro_derive** and **/use_hello_macro**.  The crate, **/hello_macro** will define a trait named `HelloMacro` with an associated function named `hello_macro`.  Rather than have crate users implement the `HelloMacro` trait for each other their, we will provide a procedural macro so users can annotate their types with `#[derive(HelloMacro)]` to get a default implementation of the `hello_macro` function.

Notice that both **/hello_macro** and **/hello_macro/hello_macro_define** are library crates.  Within the **/hello_macro** crate, we define the `HelloMacro` trait with its associated function: `hello_macro`.  At this point, a user could implement the trait however they like.  This is not desirable, however, as a user would have to write an implementation block for each type they wanted to use with `hello_macro`.  Ultimately, `hello_macro` should print: `"Hello, Macro! My name is #name"` where `#name` is the type name of the type the trait is implemented on.  Rust does not have reflection capabilities, so it cannot look up the name at runtime.

The next step is to define a procedural macro.  At the time this is being written, procedural macros require their own crates.  The naming convention for defining procedural macros is `foo_derive` for a crate named `foo`.  The **hello_macro_derive** crate needs to be specified as a procedural macro crate which can be done in **hello_macro_derive/Cargo.toml**:
```
[lib]
proc-macro = true
```
After this, it is time to define the procedural macro which is done inside of **/hello_macro_derive/src/lib.rs**.

Panicking on errors is necessary in procedural macro code because `proc_macro_derive` functions _must_ return `TokenStream` intead of `Result`.  This is why in `hello_macro_derive/src/lib.rs`, `.unwrap()` is used.

##### Attribute-like Macros
Attribute-like macros are similar to custom derived macros, but instead of generating code for the `derive` attribute, they allow for the creation of new attributes.  `Attribute-like` macros are also more flexible, `derive` only works for structs and enums; attributes can go on other items as well, like functions.

As an example of using an attribute-like macro, for an attribute named `route` that annotates functions when using a web application framework:
```
#[route(GET, "/")]
fn index() { ... }
```
This `#[route]` attribute would be defined by the framework itself as a procedural macro.  The macro definition function's signature would look something like:
```
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream { ... }
```
In the above definition, there are two `TokenStream` type parameters.  The first is for the contents of the attribute (the `Get, "/"`).  The second is the body of the item the attribute is attached to: `fn index { ... }` in this case.

Aside from these initial steps, attribute-macros work the same was as custom derived macros.

##### Function-like macros
Function-like macros define macros that look like function calls.  The syntax for defining a function-like macro is similar to that of a custom derived macro: we get in the tokens that are inside the parentheses after the `!` and return the code we want to generate.

### Project 3
#### Building a Single-Threaded Web Server
There are two main protocols involved in web servers: _Hypertext Transfer Protocol (HTTP)_ and _Transmission Control Protocol (TCP)_.  Both protocols are _request-response_ protocols meaning a client initiations requests and a server listens and provides response to the client.  The content of the messages passed between client and server are defined by the protocols.

TCP is the lower-level protocol that describes the details of how information gets from one server to another but doesn't specify what that information is.  HTTP builds on top of TCP by defining the contents of the requests and responses.  HTTP typically sends its data over TCP.  Here, we will work with raw bytes of TCP and HTTP requests and responses.

##### Listening to the TCP Connection
To begin listening for TCP connections, use the Rust crate: `TcpListener`.  Using `TcpListener`, call the `bind` method to listen on **localhost:7878**.  The `bind` function returns a `Result<T, E>`.  The `incoming` method on `TcpListener` returns an iterator that gives a sequence of streams of type `TcpStream` where a single stream represents an open connection between client and server.  A _connection_ is the name for the full request and response process in which a client connects to the server and the server generates a response and then the server terminates the connection.

When running a bare bones `TcpListener::bind("...")` you may see that multiple streams are connected to.  This could happen as a result of multiple resources getting loaded for the same page (potentially including _favicon.ico_).  Another reason multiple streams are detected could be that the browser is trying to connect to the server multiple times because the server is not responding with any data.  When elements of `TcpListener::bind("...").incoming()` go out of scope, `drop` will close the connection.

##### Reading the Request
Create a new function, `handle_connection`, which serves to read and write to individual streams.  Notice that the parameter of `handle_connection`, `stream`, is mutable.  This is because `stream` maintains an internal record of what it has returned and may read data after it has been initially checked.  The potential internal state changes call for a mutable access.

The next thing `handle_connection` does is create a buffer which is 512 bytes large.  This is done in: `let mut buffer = [0; 512];`.  After this buffer has been filled by the stream, it is converted to a string using `String::from_utf8_lossy` which takes a `&[u8]` and produces a string from it.

##### Writing a Response
An HTTP response has the following format:
```
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```
The first line is a _status line_ that contains the HTTP version used in the response, a numeric status code that summarizes the result of the request, and a reason phrase that provides a text description of the status code.

#### Turning Our Single-Threaded Server into a Multithreaded Server
Initially, our server could only handle one connection at a time meaning until the first connection finished, a second one could not be processed.

##### Improving Throughput with a Thread Pool
A _thread pool_ is a group of spawned threads that are waiting and ready to handle a task.  When the program receives a new task, it assigns on of the threads in the pool to the task, and that thread will process the task.  The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing.  When the first thread is done processing its task, it's returned to the pool of idle threads, ready to handle a new task.  A thread pool allows you to process connections concurrently, increasing the throughput of the server.
