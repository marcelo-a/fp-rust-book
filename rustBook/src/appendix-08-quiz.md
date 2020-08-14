# Test Your Knowledge

After reading over the suggested passages, we would like to test your
comprehension with a few simple questions.

## What Is Ownership?

Rust’s central feature is *ownership*. Although the feature is straightforward
to explain, it has deep implications for the rest of the language.

### Question 01 - Basic Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we
work through the examples that illustrate them:

* Each value in Rust has a variable that’s called its *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

```rust
let s = "hello";
let r1 = 5;

let r2 = s;
let r1 = 7
```

#### Move Semantics

Multiple variables can interact with the same data in different ways in Rust.
What is printed to stdout after executing the code below?

```rust
{{#rustdoc_include ../question_listings/q00/src/main.rs}}
```

<span class="caption">Listing 4-2: Assigning the integer value of variable `x`
to `y`</span>

Now let’s look at the `String` version:

```rust
{{#rustdoc_include ../question_listings/q01/src/main.rs}}
```

What is the final value owned by `r`?

<span type="text/javascript" class="caption">Listing 4-5: Returning ownership of parameters</span>

Once all questions have been answered, remeber to click `submit` to send your answers.

Thank you for your participation!

[data-types]: ch03-02-data-types.html#data-types
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#method-syntax
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
