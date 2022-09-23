# Type-Driven Rust API

This project a simple example of type-driven approach for designing an
API in Rust Language. At first glance it looks not that easy, especially if
you don't familiar with Rust and it syntax and there is a lot of syntax that is
specific only to Rust.

I took this code from this awesome video: [_"Type-Driven API Design in Rust" by Will Crichton_](https://youtu.be/bnnacleqg6k). In this video author talks about, you
guessed it, type-driven API and walks over all iterations of designing API from simple
generic function to stateful struct using traits and rich type-system of Rust.

So, if you want to understand what is going on here, I recommend to go watch the video above. Because here, i first of all myself try to understand all the details by writing
this blob of text and only after that make this blob understandable by other people.
And if I achive both this targets, then I made a good job at understanding this design
pattern, good me!

## Progress struct

Firstly, let's ignore the first three lines of code in file `src/main.rs`, because they
not related to the topic. So, first what you see in file is this:

```rust
struct Unbounded;

struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}
```

First what you notice is the `Unbounded` struct. And you may ask, why it don't have body
with fields? Is this allowed? Yes, it's allowed and in Rust this type of struct is
called a `Unit struct`. Unit structs are most commonly used as marker. They have a
size of zero bytes, but unlike empty enums they can be instantiated, making them
isomorphic to the unit type `()`. Unit structs are useful when you need to implement a
trait on something, but don't need to store any data inside it. And that is exact
purpose of `Unbounded` struct.

Let's skip for now the `Bounded struct` and talk firstly about `Progress` struct. This
is the main struct of this small project. And all what it does is printing to standard output a progress of iterator and it looks something like this:

```console
[***   ]
```

Nothing fancy, but you also can customize it by changing the bound brackets or
removing them at all (and by the default they turned off). So, the progress bar can
look this:

```console
<***   >

{***   }

***
```

And for this exact behavior we have the `Bounded` struct (actually not only for this).

## Using the Progress struct

Before talking about the implementation of `Progress`, let's look at how you can use it
in yout code. For exapmle, we have this simple program where we just iterating over
vector of integers, doing some expensive calculations on it's values:

```rust
fn main() {
  let valuable_data = vec![1, 2, 3, 4, 5];

  for data in valuable_data.iter() {
    expensive_calculation(data);
  }
}
```

And the problem with this that we don't know what is the state of the program. Is it
doing something? Is it stuck? We don't know. And to make our program more informative
all we have to do is just add another call to `progress()` method after `iter()` with
dot notation, so our code now will look like this:

```rust
fn main() {
  let valuable_data = vec![1, 2, 3, 4, 5];

  for data in valuable_data.iter().progress() {
    expensive_calculation(data);
  }
}
```

```console
***
```

And look, now we know that our program is doing something! But it looks kinda ugly. It
just prints `*` to the terminal, maybe we can go little bit fancier? Yes, we can, and
to add some style we need another method call `with_bound()`:

```rust
fn main() {
  let valuable_data = vec![1, 2, 3, 4, 5];

  for data in valuable_data.iter().progress().with_bound() {
    expensive_calculation(data);
  }
}
```

```console
[***  ]
```

That's looks more like a progress bar, nice! But maybe I don't like square brackets
and what it to display bar with pipes? For this, we can add another method call, where
in parameters we can pass desirable characters for bar delimiters `with_delims((char, char))`:

```rust
fn main() {
  let valuable_data = vec![1, 2, 3, 4, 5];

  for data in valuable_data.iter().progress().with_bound().with_delims(('|', '|')) {
    expensive_calculation(data);
  }
}
```

```console
|***  |
```

Nice, the progress bar now looks like we wanted. But the code looks kinda ugly,
let's refactor it a little bit:

```rust
fn main() {
  let valuable_data = vec![1, 2, 3, 4, 5];

  valuable_data.iter()
    .progress()
    .with_bound()
    .with_delims(('|', '|'))
    .for_each(|data| expensive_calculation(data));
  }
}
```

And that what I call an art! But if you familiar with Rust, you can ask, what about the
unbounded ranges `(2..)`? What if I call `(2..)iter().progress().with_bound()`? Will
it be an undefined behavior? And the answet is... it won't even compile! There just no
method attached to iterators that don't have exact size. And all it because of
superpowers that give to you Rust's _traits_!

## How it works (todo)

Let's start from simple and look at first two `impl` blocks of `Progress` struct:

```rust
impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}
```

Let's take it bit by bit. The `impl<Iter>` bit is saying that this `impl` block
implemented all types of `Iter` and Iter isn't some concrete type name, it's a generic
so it could be named like `T` or `Type` or _etc_.

The `Progress<Iter, Unbounded>` bit is just attaches types `Iter` and `Unbounded` to
`Progress` struct. And unlike `Iter`, `Unbounded` is a concrete type or more exactly _unit struct_, that we defined earlier. And inside the `impl` block is just a
constructor function `new()` that takes as argument any type.

The next `impl` block looks almost the same, only just with another function. But the
only and key difference that instead of `Unbounded` type it has `Bounded`. And that means that the function `with_delims()` available only for those `Progress` instances that have the `Bounded` state. And that's why (partially) we can't call
`with_delims()` method on iterators without exact size! But it the only first part
why.

And the other part of why we can't call `with_delims()` method on iterators without
exact size is because of this `impl` block:

```rust
impl<Iter> Progress<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };

        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}
```

It's looks a little bit more involved, but the only difference from first `impl` is
the `where` clause. And line `where Iter: ExactSizeIterator` just means that the
`with_bound()` method only available for those types that implemented [`ExactSizeIterator`](https://doc.rust-lang.org/std/iter/trait.ExactSizeIterator.html) trait. Pretty cool if you ask me.

The next `impl` block looks like this:

```rust
impl<Iter, Bound> Iterator for Progress<Iter, Bound>
where
    Iter: Iterator,
    Bound: ProgressDisplay,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{CLEAR}");
        self.bound.display(&self);

        self.i += 1;
        self.iter.next()
    }
}
```

And there is some \`magic outside Hogwarts\`. So shortly, it's an implementation of
[`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait defined in
Rust's standard library. And by implementing it's required method `next()` the struct
`Progress` can be turned into a iterator! And other thing about this is the `Bound`
generic and `ProgressDisplay`.

So, the line `Bound: ProgressDisplay` allow us to call `display()` method on `bound`
field of `Progress` struct. And this method defined in `ProgressDisplay` trait allow
us to implement different behaviors of `display()` function on different states of
`Progress` struct. and the code that implements all of this:

```rust
trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i))
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(progress.i),
            " ".repeat(progress.bound.bound - progress.i),
            self.delims.1
        )
    }
}
```

And the last thing we left to talk about is how with traits we can extend functionality
of predefined public interfaces of standard library and pretty mach any library!

Firstly, let's look at the code:

```rust
trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}
```

In beginning we defining the `ProgressIteratorExt` trait and associated method
`progress`. Then, we implementing it for every type that implements the
[`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait. And
this allow us to make this call `vec![1, 2, 3].iter().progress()`. Pretty awesome
if you ask me!

## Conclusion

I think Rust is very powerful tool that allow you do things that you can't do in
other languages, or you can do but the implementation and methods achieving this
kind of behavior will be much messier. Also Rust is very fun to work with and it
naturally motivates you to write beautiful and clean code.

So, I hope this little essay ended up somewhat readable for someone else and only for
me, I really tried.
