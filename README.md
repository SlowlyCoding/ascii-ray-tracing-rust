# ASCII Ray Tracing in Rust

A simple ray tracer inspired by the "Ray Tracing in one weeked" book, which display the rendered scene using ascii.
The camera can be moved around inside the scene.


<p align="center">
<img src="https://github.com/SlowlyCoding/ascii-ray-tracing-rust/blob/master/gifs/video.gif">

I had already written something similar in C++, but now im learning Rust and thought I could rewritte that project to learn more about the Rust programming language.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
2. [Configuration](#configuration)

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed

```shell
git clone https://github.com/SlowlyCoding/ascii-ray-tracer
cd ascii-ray-tracer
cargo run
```

## Usage

The camera can be moved around

`W A S D` - forwards, left, backwards, right

`Q E` - up, down

`← → ↑ ↓` - tilt camera

`+ -` adjust camera FOV


## Configuration

You can make your own scene and change some options by adjusting the scene on line 24 inside `src/main.rs`
