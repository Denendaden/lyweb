# LySSG - lyra's static site generator <3
This is a static static generator written in Rust that I created in the process
of making my personal website, <https://lyra.pink>.
It is very simple and not very featureful, but I decided to publish it
separately from my website in the hope that maybe someday someone will find it
useful for making their own website.

## How it works
LySSG works by modifying a template with some provided content. A template file
might contain the following:
```html
...
<div id="contents">
    [[content]]
</div>
...
```
LySSG provides functions to replace the text `[[content]]` with some content.
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

## Planned features
- [x] Navigation bars in templates that disable links to the page you are on
- [x] Conversion from Markdown to HTML to make creating pages easier
- [ ] Some kind of blog system that can show lists of recent posts
- [ ] Caching to avoid unnecessarily regenerating webpages
