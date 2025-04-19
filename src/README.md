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
