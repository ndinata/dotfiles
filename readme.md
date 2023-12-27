# dotfiles

First run `init.sh`, which will print out more instructions on what to do next.
```sh
$ bash init.sh
```

If you encounter issues when building the `drip` cli app (last step of `init.sh`),
try the following:
```sh
# Install `llvm` to use `lld` as the default linker
$ brew install llvm
$ fish_add_path /opt/homebrew/opt/llvm/bin

# Install the cli app again
$ cargo install --path drip
```
