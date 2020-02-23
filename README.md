# Fuzzle - Fast fuzzy application launcher

Fuzzle aims to be a really fast application launcher with fuzzy search.

![screen](screen.png)


# Usage

Build with:
```
cargo run --release
```

The first time Fuzzle runs it takes some time (less than a second on my pc) to build a cache of existing applications.

After that, it's supposed to run as fast as you can type.

Write something to filter results, use Ctrl+j or ArrowDown and Ctrl+k or ArrowUp to go through the results, press Enter to open the selected application or Esc to exit Fuzzle.
