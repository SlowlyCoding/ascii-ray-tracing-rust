# ASCII Ray Tracing 

A simple ray tracer inspired by the "Ray Tracing in one weeked" book, which display the rendered scene using ascii.
The camera can be moved around inside the scene.
I had already written something similar in C++, but now im learning Rust and thought i could rewritte that project to learn more about the Rust programming language.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
2. [Configuration](#configuration)

## Installation

Make sure you have [rust](https://www.rust-lang.org/tools/install) installed

```shell
git clone https://github.com/SlowlyCoding/ascii-ray-tracer
```
```shell
cd ascii-ray-tracer
```
```shell
cargo run
```

## Usage

The camera can be moved around

`W``A``S``D` - forwards, left, backwards, right
`Q``E` - up, down
`←``→` - tilt camera left, right
`↑``↓` - tilt camera up, down

## Configuration

You can make your own scene and change some options by adjusting the scene on line 24 inside `src/main.rs`
