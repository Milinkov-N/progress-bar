# Type Driven Rust API

This project a simple example of type driven approach for designing an
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

First what you notice is the `Unbounded` struct. And yoy ask, why it don't have body
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

That's looks more like a progress bar, nice! But maybe i don't like square brackets
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
