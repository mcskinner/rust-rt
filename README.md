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

# Chapter 2

### New type for Color

The alternative is to reuse some base implementation for both Tuple and Color.

I opted for a new type, since the derived traits do most of the work anyway. The names are convenient enough to keep.

### Dealing with the PPM formatting specifics

I was tempted to just write one pixel per line, because it's simple and also follows the 70 character constraint. I also considered keeping colors together, not splitting them with line breaks.

In the end I stuck with fidelity to the test case as written in the book, as a challenge to see if I could write it cleanly.

It's okay, not my favorite bit of code. In a real production system I'd bias towards changing the tests to be less prescriptive, i.e. only checking the 70 character constraint and pixel ordering correctness, without so much concern for exactly where the line breaks are. That is, the test should cover the boundaries, but should not introduce tighter implementation constraints than strictly required (as this one does).

# Chapter 3

### Types for matrices

In this case I had to go with `Vec<Vec<f64>>`, since arrays are fixed size and NxM individual variable names gets out of hand.

I didn't realize there was an `ndarray::arr2` I could have used until I was halfway through. There's even a [cookbook for linear algebra](https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html) that uses it to cover what I've been doing the harder way.

I'll continue with my by-hand implementation, but will consider refactoring later.

# Chapter 4

### Where to put the transformations feature

I think it makes the most sense to put these on the Matrix object itself.

The case for a new file is that these aren't things a matrix does, but rather a use of a matrix to do something.

But in practice it'll be nice to have everything in one place.

# Chapter 5

### How to track intersection objects

The way the compiler recommended was to implement and use clone. And that's fine, I guess.

But I think intersections are going to be common and I don't want them to be too heavyweight.

So I decided to go with borrowing, and had to use some only-partially-understood lifetime annotation syntax with <'a> and all that to make it work.

### Sphere equality

I went with a pointer equality check I found on S/O.

### Intersection list

Going with a plain `Vec<Intersection>` and will make `hit` a plain function.

I've done everything object oriented so far, but maybe that's not the best fit for the language.

I actually had to refactor this later to an `Intersections` object of its own. The type management got out of hand with the plain `Vec`.

### Moving toward borrowing

I keep getting warnings about cloning. I'd probably have formed a habit of slapping Clone and Copy derives on everything if Vec had allowed the implicit.

But it doesn't, which I took as a nudge to do more borrowing.

As it has increasingly become the standard for me, I'm going to make it my default going forward. It'd probably be worthwhile to go back and refactor earlier code, but not sure when I'll take that on. The initial 400x400 renders have been fairly slow though (many seconds) which suggests to me some meaningful room for improvement. Maybe worth looking into a profiler.

# Chapter 6

### New file per type

I am doing this more and more, it's not that much overhead to have a file and it's nice to keep things separate.

### Go back and refactor?

This chapter was straightforward, but I'm noticing more and more patterns I used early that I'd reverse now.

- Constructor functions outside of the impl. First up is to move `color` to `Color::new`.
- How to wrap types / use the newtype pattern. I think newtype is probably what I wanted, but didn't grok the `.0` syntax for accessing the wrapped variable.
- Importing functions inside the test submodule, which Copilot just did. This makes a lot of sense, probably a bunch I can port there.
