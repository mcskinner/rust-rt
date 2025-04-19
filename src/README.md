# Rust Ray Tracer (rust-rt)

Rust implementation of The Ray Tracer Challenge, and my personal deeper dive into Rust.

I am going to use this README as a log of decisions made along the journey.

## Chapter 1

### Type for tuple

Rust has a [tuple type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type). But it's meant for types to vary within the tuple.
The [array type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type), on the other hand, has a consistent type throughout and a fixed length.
The last choice is a [struct type](https://doc.rust-lang.org/book/ch05-01-defining-structs.html). The advantage is you can name the variables, which is convenient, but it allows you to eschew ordering, which is strictly incorrect.

I started to go with the array type, with a consistent convention of [named destructuring](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) when necessary e.g. for testing.

I was betting that destructuring would be rare relative to pure mathematical iteration over the data structure, as is most common with vectors.

But I tried it, and the type system doesn't really seem built for this. So I'm going to a basic struct type.

### Methods or operator overloading

Overloading all the way, especially for this context. And Rust seems to make it super easy.

### Clone or Copy

I needed these for normalization. Normally I go for the more restrictive option, which in this case is Clone.
But it's allowed to be arbitrarily expensive in implementation, whereas Copy is implicit and uses memcpy.

Easy choice, default to Copy unless there's a strong reason to do otherwise. But implement both as a habit, I guess.

### Endnotes

I'm surprised how much Github Copilot is doing for me. I mean, I guess not really since this has been out there awhile.
And I gotta say, it's a pleasant surprise. I've spent enough time keying in code from books over the years.
This way I can focus on reviewing the correctness and learning the language idioms at a higher level.
Like I said, pleasant surprise.
