use std::fmt;


/// A debug-printable struct.
#[derive(Debug)]
struct DebugPrintable(i32);

/// Going deeper.
#[derive(Debug)]
struct DeeperStruct(DebugPrintable);

/// I have no idea what I'm doing.
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

#[derive(Debug)]
struct DisplayableStruct {
    left: i32,
    right: i32
}

impl fmt::Display for DisplayableStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({left}, {right})", left = self.left, right = self.right)
    }
}

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }

            write!(f, "{}: {}", i, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    println!("Hello world!");
    println!("4+4={}; 5*6={}", 4+4, 5*6);
    println!("4+4={1}, 5*6={0}", 5*6, 4+4);
    println!("{hello}, {world}{exclamation}", hello="Hello", world="world", exclamation="!");
    println!("4+4 in binary is {0:b}", 4+4);
    println!("4+4 in debug mode is {0:?}", 4+4);
    eprintln!("There's been an error!");

    let pi = 3.141592;
    println!("Pi is roughly {:.width$}", pi, width=3);

    let debuggable = DebugPrintable(3);
    println!("Debugging struct: {:?}", debuggable);

    let deeper = DeeperStruct(DebugPrintable(3));
    println!("Debugging deeper struct: {:?}", deeper);

    let person = Person { name: "Rustickus", age: 136 };
    println!("Behold the great Rustickus: {:?}", person);

    let displayable_struct = DisplayableStruct { left: 123, right: 456 };
    println!("Displayable struct with debug formatting: {:?}", displayable_struct);
    println!("Displayable struct with custom formatting: {}", displayable_struct);

    let complex_num = Complex { real: 3.3, imag: 7.2 };
    println!("Display: {}", complex_num);
    println!("Debug: {:?}", complex_num);

    let list = List(vec![1, 2, 3]);
    println!("List: {}", list);
}