fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    // NOTE Destructure by assigning a variable name to each return value
    let (name, age) = cat;

    println!("{name} is {age} years old");
}
