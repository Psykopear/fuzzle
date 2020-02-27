# Fuzzle - Fast fuzzy application launcher

Fuzzle aims to be a really fast application launcher with fuzzy search.

![screen](screen.png)

# Note
The project is in a very early stage and things will change and break often for a while.
If you are looking for a working launcher, this project is inspired by:
- [Ulauncher](https://github.com/Ulauncher/Ulauncher)
- [rofi](https://github.com/davatorium/rofi)

# Usage

Build with:
```
cargo run --release
```

The first time Fuzzle runs it takes some time (less than a second on my pc) to build a cache of existing applications.

After that, it's supposed to give results as fast as you can type.

- Write something to filter results
- Use Ctrl+j or ArrowDown and Ctrl+k or ArrowUp to go through the results
- Press Enter to open the selected application
- Press Esc to exit Fuzzle.
