# yew-data-fetch

## spec

use [HackerNews API](https://github.com/HackerNews/API).

The endpoint is https://hacker-news.firebaseio.com/v0/item/8863.json?print=pretty.

## dev

```
wasm-pack build --target web --out-name wasm --out-dir ./static

miniserve ./static --index index.html
```
