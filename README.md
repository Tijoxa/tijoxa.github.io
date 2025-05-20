# my website
first test of github pages functionnality

https://tijoxa.github.io/

## doc
setup wasm-pack: https://rustwasm.github.io/docs/wasm-pack/introduction.html

setup github pages: https://pages.github.com

setup gh actions for gh pages: https://github.com/peaceiris/actions-gh-pages

for local, run
```Bash
wasm-pack build --release --target web; uv run python -m http.server --bind 127.0.0.1 8000
```
