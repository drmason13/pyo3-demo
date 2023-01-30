# Instructions

These instructions are written to be run from the root of this demo project. I.e. you should see something like this:
```sh
~/git/pyo3-demo$ ls
README.md  examples/
```

You can see the output for each of the sections below in its own folder inside examples/

## 1. Getting Started: Hello World - `examples/hello_world`

See the [PyO3 User Guide](https://pyo3.rs/v0.18.0/getting_started) which served as the source of information and inspiration for this walkthrough.

- `mkdir hello_world`
- `cd hello_world`
- `python -m pip install maturin`
- `maturin init` and then select "pyo3 bindings"
- Then you can create your Rust library and make it callable from python by using the `#[pyfunction]` and `#[pymodule]` macros. See `examples/hello_world/src/lib.rs` for an example, edit your `src/lib.rs` so it matches.
- Run `maturin build` to build and compile your python module into a wheel file.
- Run `python -m pip install <path to wheel>` to install the wheel so it can be run imported by python. The `<path to wheel>` is from the output of `maturin build`, e.g.
```
 maturin build
🔗 Found pyo3 bindings
🐍 Found CPython 3.10 at /usr/bin/python3
   Compiling pyo3-demo-repro v0.1.0 (/home/user/git/pyo3-demo/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
📦 Built wheel for CPython 3.10 to /home/user/git/pyo3-demo/hello_world/target/wheels/hello_world-0.1.0-cp310-cp310-manylinux_2_34_x86_64.whl
```
- Try it!
```py
python
>>> import hello_world
>>> hello_world.greet("World")
'Hello World'
```

## 2. Practical Example: Classes - `examples/classes`

- Make sure you've run through Hello World first so that you have maturin installed. If you're using a venv, install maturin into your venv for each project.
- `mkdir classes`
- `cd classes`
- `maturin init` and then select "pyo3 bindings"
- copy the content of `examples/classes/src/lib.rs` to your src/lib.rs

This contains a python class defined in Rust as a struct [struct](https://doc.Rust-lang.org/stable/book/ch05-00-structs.html).
The fields belonging to the class and their types are written like so:
```rs
#[pyclass]
struct ExampleClass {
    #[pyo3(get, set)]
    value: i32,
}
```

`value` is the name of the field (a python instance attribute). The `#[pyclass]` and `#[pyo3(get,set)]` tell pyo3 to make this class available to Python and that the attribute is both readable and writable, you don't need to worry too much about how that works exactly.

the `: i32` in `value: i32` is the type declaration. `i32` is a sized 32-bit integer which maps well to python's `int` type.

So our `ExampleClass` has one field named value that holds an integer. That's that.

"Where are the methods" you ask? Fair question, in Rust, class (struct) methods are held in an "impl block". `impl` is a keyword short for implementation. Here's the code:

```rs
#[pymethods]
impl ExampleClass {
    #[new]
    pub fn new(value: i32) -> Self {
        ExampleClass { value }
    }

    ... other methods ...
}
```

* It starts with `impl ExampleClass` which says "we're implementing methods for ExampleClass".
* `fn` declares a function, just like `def` in Python. `pub` 
* backtracking slightly, the `pub` means `public`, this function we're about to define is part of the struct's public interface (as opposed to a private interface which is the default)
* `new(value: i32) -> Self` is the function signature: `new` is the name. In brackets you've got the arguments and their type (types are required in every function signature in Rust!) and `-> Self` is the return Type. `Self` means it returns the same type as the struct. In this case that means it returns an `ExampleClass`.
* As you may have guessed `new` is to Rust what `__new__`\* is to Python. Although this is just by convention in Rust. Interstingly enough there's nothing actually magic about the `new` function in Rust, any other named function with the same signature would work just as well.

\* In Python, `__new__` is basically one step before `__init__` and like the new function we see in Rust, it doesn't take a `self` argument (it's a staticmethod). It's job is to actually create an instance of the class. `__init__` then takes this new instance and "initialises" it (hence init). Rust doesn't let you separate making the instance and initialising it: It's impossible to create an uninitialised instance of a struct in Rust.

OK, we're finally in the function body!

There's a few things to talk about here:

1. no `return` statement, the final expression of the function is returned for you.
2. `ExampleClass { value }` is instantiating the struct (and returning it). It sets the `value` field to the value of the `value` function argument.

The whole struct declaration and the `new()` method declaration is basically equivalent to this python code:

```py
class ExampleClass:
    def __init__(self, value: int):
        self.value = value
```

Oh and, the `#[pymethods]`  nd`#[new]` are pyo3 things. `#[pymethods]` you don't need to worry about much, put it above every `impl MyClass {` block as this tells pyo3 what methods belong to your Python class. The `#[new]` tells pyo3 which method to use as `__new__()` (pyo3 does

Finally, there's a method called `double` which doubles the currently held value. It takes `&mut self` as a parameter which is similar to the self parameter in a python method. `&mut` means it's a "mutable reference" to self which means it lets you "mutate", i.e. change things!

A warning: For demonstration purposes, this simple little class is fine but it can break! Try this:

```
>>> x.value = 1
>>> for _ in range(33):
...     x.double()
...
thread '<unnamed>' panicked at 'attempt to multiply with overflow', src/lib.rs:18:22
Traceback (most recent call last):
  File "<stdin>", line 2, in <module>
pyo3_runtime.PanicException: attempt to multiply with overflow
```
uh oh! what happened here? Well, you know the `i32` type we used for the value? Well, it's not exactly like a python int, it really only holds 32-bit integers. You double 1 33 times and that's bigger than a 32-bit integer can represent. In this case Rust will "panic" to be on the safe side. Unfortunately, though it might look like pyo3 has handled this panic for us quite nicely.
Pyo3 actually raises a BaseException which, like SystemExit will usually halt the interpreter. And while it is possible to `except BaseException:` Rust will still have already panicked which halts any Rust code completely and prints a message about panicking. The best thing to do is to avoid panicking in your Rust code in the first place.

To fix this simple example I would use [`checked_mul()`](https://doc.Rust-lang.org/std/primitive.i32.html#method.checked_mul) instead of simply multiplying with `*`:

```rs
    fn double(&mut self) -> PyResult<()> {
        self.value = self.value.checked_mul(2).ok_or(PyValueError::new_err(
            "Value is too large and would overflow if doubled",
        ))?;

        Ok(())
    }
```

This also needs an extra import:

```rs
use pyo3::exceptions::PyValueError;
```

Now it will raise a Python ValueError Exception when doubling too many times, this is easily handled from python:

```
>>> x.value = 1
>>> for i in range(33):
...     try:
...         x.double()
...     except ValueError:
...         print("Whoops! too big")
...         print(f"x = {x.value} after {i} doubles")
...         break
...
Whoops! too big
x = 1073741824 after 30 doubles
>>> exit()
```
and what do you know, didn't need all 33 doubles to overflow an i32 after all.

## Uninstall:

* To uninstall these demo packages, just use pip, e.g: `python -m pip uninstall hello_world`

## Troubleshooting

This guide was written and tested on a WSL2 Ubuntu system:
`Linux Desktop 5.10.102.1-microsoft-standard-WSL2 #1 SMP Wed Mar 2 00:30:59 UTC 2022 x86_64 x86_64 x86_64 GNU/Linux` using `Python 3.10.6` and `Rustc 1.65.0 (897e37553 2022-11-02)`. This part of the pyo3 guide might be of some help: https://pyo3.rs/main/building_and_distribution
