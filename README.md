# Day 1: Trebuchet?!

## Part 1

First we take care of some boilerplate Rust. We'll import everything we need to read a file to a string buffer and set our main function up to do that. I use `Box<dyn Error>` as an inefficient and lazy catchall because I'd rather use the `?` operator than do any real error handling. At this point we'll also split the input by line and clean them so we can iterate through the collection.

```rust
use std::fs::File;
use std::io::prelude::*;
use std::error:Error;

fn main() -> Result<(), Box<dyn Error>> {
	// Open our input file and read it to a String
	let mut file = File::open("src/input")?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;

	let v: Vec<&str> = buffer.trim().split('\n').collect();

	Ok(())
}
```

Now lets take a look at some of our input and think about what is expected of us. Here are the top six entries of my input:

```
eight9fhstbssrplmdlncmmqqnklb39ninejz
three656
ppjvndvknbtpfsncplmhhrlh5
7fjqhrhsevenlbtwoninevnmct2
qjnbpfrztwo1
plggqjthree49four
```

We want to find the first and last instance of a digit on each line. If there is only one digit, it is both the first and last digit. They must be in first-then-last order. Here is what we should expect:

```
99
66
55
72
11
49
```

Since we're going to be working with `&str` primitives, I started looking at the rust documentation and found a `find` and `rfind` method which appeared to be just what I needed to find the first and last ascii digit.

```rust
...
fn main() -> Result<(), Box<dyn Error>> {
	...
	let v: Vec<&str> = buffer.trim().split('\n').collect();

    for line in v {
        let i = line.find(|c: char| c.is_ascii_digit()).unwrap(); 
        let first = line.get(i..i+1).unwrap().parse().unwrap();
    }

	Ok(())
}
```

As you might see from the multiple uses of `unwrap()`, we're going to make a few unsafe assumptions about the data we're giving the program. Our program assumes each line will have an ascii digit and then assumes that digit is represented in a single character. We can tidy up the safety later. For now, we can print `first` and see that it correctly interprets our input and gives the result we'd expect from each line. Now we need only implement the same thing using `rfind` to find the last instance of each number. With the first and last digit parsed, we can create our two digit number by multiplying the first number by 10 and adding it to the last. We then just add these to a grand sum. Here's the complete final solution for part 1:

```rust
use std::fs::File; 
use std::io::prelude::*;
use std::error::Error; 

fn main() -> Result<(), Box<dyn Error>> {
    // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    
    let v: Vec<&str> = buffer.trim().split('\n').collect();
    
    let mut result = 0;
    
    for line in v {
        let i = line.find(|c: char| c.is_ascii_digit()).unwrap(); 
        let first: i32 = line.get(i..i+1).unwrap().parse().unwrap();
        let i = line.rfind(|c: char| c.is_ascii_digit()).unwrap(); 
        let last: i32 = line.get(i..i+1).unwrap().parse().unwrap();
        result += first * 10 + last;
    }
    
    println!("Final result: {:?}", result);

    Ok(())
}
```

There's a number of improvements that can be made for the sake of efficiency and safety. However, for our purposes, this code works perfectly and we can move on to part 2.

## Part 2

<img src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fc.tenor.com%2FTTQF7ubqYbsAAAAC%2Fstumble-shake.gif&f=1&nofb=1&ipt=c0a62417621ae46a6888149b8a0168e65da59c315b3ac50bbecd0989584c925e&ipo=images">
