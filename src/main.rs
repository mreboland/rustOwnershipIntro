fn main() {
    println!("Hello, world!");

    // In terms of ownership, Rust makes the following pair of promises, both essential to a safe systems programming language.
    // 1. We decide the lifetime of each value in our program. Rust frees memory and other resources belonging to a value promptly, at a point under our control.
    // 2. Our program will never use a pointer to an object after it has been freed. Using a dangling pointer is a common mistake in C and C++, wherein, if we're lucky our program crashed. If unlucky, our program has a security hole. Rust catches these mistakes at compile time.

    // In Rust, every value has a single owner that determines its lifetime. When the owner is freed (dropped), the owned value is dropped too. These rules are meant to make it easy for us to find any given value's lifetime simply by inspecting the code, giving us the control over its lifetime that a systems language should provide.

    // A variable owns its value. When control leaves the block in which the variable is declared, the variable is dropped, so its value is dropped along with it. For example:
    fn print_padovan() {
        let mut padovan = vec![1,1,1]; // allocated here

        for i in 3..10 {
            let next = padovan[i-3] + padovan[i-2];
            padovan.push(next);
        }

        println!("P(1..10) = {:?}", padovan); // dropped here
    }
    // The type of the variable padovan is std::vec::Vec<i32>, a vector of 32-bit integers.

    // Rust's Box type serves as another example of ownership. A Box<T> is a pointer to a value of type T stored on the heap. Calling Box::new(v) allocates some heap space, moves the value v into it, and returns a Box pointing to the heap space. Since a Box owns the space it points to, when to Box is dropped, it frees the space too.
    // For example, we can allocate a tuple in the heap like so:
    let point = Box::new((0.625, 0.5)); // point allocated here
    let label = format!("{:?}", point); // label allocated here
    assert_eq!(label, "(0.625, 0.5)"); // both dropped here
    // When the program calls Box::new, it allocates space for a tuple of two f64 values on the heap, moves its argument (0.625, 0.5) into that space, and returns a pointer to it. See page 125 for a visualization of the heap.

    // Just as variables own their values, structs own their fields, and tuples, arrays, and vectors own their elements:
    struct Person { name: String, birth: i32 }

    let mut composers = Vec::new();
    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Downland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
    // Here, composers is a Vec<Person>, a vector of structs, each of which holds a string and a number. See page 126 for a visualization.
    // There are many ownership relationships here, but each one is pretty straightforward. Composers owns a vector, the vector owns its elements, each of which is a Person structure. Each structure owns its fields, and the string field owns its text. When control leaves the scope in which composers is declared, the program drops its value, and takes the entire arrangement with it. This applies if there were any other sorts of collections as well like a HashMap, or a BTreeSet.

    // Every value has a single owner, making it easy to decide when to drop it. But a single value may own many other values. For example, the vector composers owns all of its elements. Those values, may own other values in turn and each element of composers owns a string, which owns its text.
    // It follows that the owners and their owned values form trees. Your owner is your parent, and the values you own are your children. At the ultimate root of each tree is a variable, and when that variable goes out of scope, the entire tree goes with it. Every value in a Rust program is a member of some tree, rooted in some variable.

    // The way to drop a value in Rust is to remove it from the ownership tree somehow. Either by leaving the scope of a variable, or deleting an element from a vector, or something of that sort. At that point, Rust ensures the value is properly dropped, along with everything it owns.

    // Rust is less powerful than other languages like C and C++ because they allow you to build whatever you want regardless of good practice. But at the same time this makes Rust safety a guarantee due to its limiting nature. Rust claims there is usually more than enough flexibility in how one goes about solving a problem to ensure that at least a few perfectly fine solutions fall within the restrictions the language imposes.
    // However, Rust does allow us some flexibility in several ways:
    // 1. We can move values from one owner to another. This allows us to build, rearrange, and tear down the tree.
    // 2. The standard library provides the reference-counted pointer types Rc and Arc, which allow values to have multiple owners, under some restrictions.
    // 3. We can "borrow a reference" to a value. References are non-owning pointers, with limited lifetimes.
    // Each of the above contributes flexibility to the ownership model while still upholding Rust's promises.
}
