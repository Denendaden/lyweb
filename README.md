# LyWeb - lyra's website library
This is a library written in Rust that I created for my personal website,
<https://lyra.pink>, containing the functionailty that seemed like it could be
reproduced for other websites.
It is very simple and not very featureful, but I decided to publish it
separately from my website in the hope that maybe someday someone will find it
useful for making their own website.

## How it works
LyWeb works by modifying a template with some provided content. A template file
might contain the following:
```html
...
<div id="contents">
    [[content]]
</div>
...
```
LyWeb provides functions to replace the text `[[content]]` with some content.
This means you can ensure consistent styling, etc. across all pages that use the
same template. An example of this in use, taken from
[my own website](https://lyra.pink):
```rust
LyWebpage::from_file("templates/main.html")?
    .fill_from_file("content", "www/content.html")?
    .contents
```
This snippet would return a `String` that could be sent in a response to a
request.
