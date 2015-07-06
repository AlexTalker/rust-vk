# rust\_vk

[![Build Status](https://travis-ci.org/AlexTalker/rust-vk.svg?branch=master)](https://travis-ci.org/AlexTalker/rust-vk)

VK.com API wrapper for Rust programming language

_Note_: Посвящается самой необыкновенной, доброй и непредсказуемой девушке на свете, которую автор когда-либо встречал...

## Description:

The library is implemented to make easily authorization and api calls to VK.com API on Rust, for example: 

```rust
extern crate rust_vk;
use rust-vk::api::VkApi;
use rust-vk::app::VkApp;
...
let app = VkApp(43727634, "v5.34", None);// Useless example for a while :)
...

```
