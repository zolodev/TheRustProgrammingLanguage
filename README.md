# TheRustProgrammingLanguage
This repo is me trying to follow the code examples of the rust book

The book can be found at 
[Rust book](https://doc.rust-lang.org/book/title-page.html)

# My personal review and verdict

## TL;DR
I rate this book 4/5

Whould I recommend this book?

Yes, but **not** to a beginner.

## Overall

Overall it is a lot to cover in a book like this. Not only did I buy a copy of
the physical book to support the authors. I found it to be easier reading it
[online](https://doc.rust-lang.org/book/title-page.html) instead.

I read everything from cover to cover over several days. I tried to do at 
least one chapter each day.

My background in development is over 10 years. I have worked as a Software
Developer with other languages like C# and Python. But started development at
the age of 12. I have read a lot of books and online material in my life.

With that said...

## My Review of the Rust book ISBN: 978-1-7185-0044-0
My first impression was very positive, it was a very good introduction.
I felt a familiarity to the approach that I knew about when I started to learn.

However from chapter 3 I started to struggle a lot with the code examples.
I never really felt that the author was explaining the intentions nor the
motivations of the examples in depth. The code examples felt like they where
always "half baked" and the author refered to another chapter.

One of the thing that was not clear for me to begin with; but it came to me in
future chapters; was the chapter regarding
[Closures](https://doc.rust-lang.org/book/ch13-01-closures.html).

This was the first time I got stuck for several hours, trying to understand
the authors intention and purpos of the code example.

I turned to the community for guidance, where I got a very good example to
understand closure. So instead of looking at Listing 13-1 (52 lines of code)
I would suggest to look at this code instead.
```rust
// A simple example of a Rust closure `|x| x + 1`
let f = |x| x + 1;
println!("f: {}", f(10));   // Result: 11 
```

One other thing I was missing was the "proof" of many of the examples.
The lack of proof that would make a different, could have done so much for my
general understanding. Several examples in the book are missing "the proof".
Much of the examples is very theoretically and hypothetical. In other words, 
not very beginner friendly. Most of the time I had to add my own proof. 
To verify that I understand how to use the implementation of the code example.

## Verdict

I could figure out many of the theoretical code and discussion, I had a lot of
problem understanding why. While Other chapters where very well written and
gave a good explaination that I enjoyed reading.

**I would rate this book 4/5:**

**The good**
* It is free and open source!
* Rust is a lot to cover in one book, many new concepts that are new and I 
think it is important to mention that I find the author i many ways good at 
explaining the new concept in a good way.
* The example projects where very good, easy to follow and explained in a good
way. In my opinion they where the reason I wanted to know more.

**The bad**
* For refering to other chapters "in the future". Why not explain it while we
are on the topic?
* For lacking a lot of proof, to show that the code (that compiles) works. I 
miss more examples of usage of some parts. To get the verification how to use
the code.
* I find my self wondering more about the code and the motiviation instead of
understanding what the author is trying to convey. By bad code examples that 
are to verbose. Where there is more simplier ways to explain code with.
* The worst thing to teach is bad practices! No production ready code is bad
examples. This is not only dangerous to teach but could lead to bad code.
If I could change one thing in the book I would recommend the author to fix
all the code to be production ready with good explainations (proof).
* This is not a book for beginners, this might be a good book when you worked
several months with the Rust language. To get a deeper understanding of Rust.



**Would I recommend the book?**

Yes, but **not** to a beginner.
