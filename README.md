# IoResultOptional

A trait for [io::Result](https://doc.rust-lang.org/std/io/type.Result.html)
that adds a method making it easy to tell the difference between a
file not found and another error, since a common practice is to handle
a file if it exists.

[![Build Status](https://travis-ci.org/kaj/io-result-optional.svg?branch=master)](https://travis-ci.org/kaj/io-result-optional)

````rust
if let Some(input) = File::open("data").optional()? {
    // The data exists, so handle it ...
    // If it doesn't exist, it is just ignored
    // If there is another error, this function returns it.
}
````
