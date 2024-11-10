# `Mass Move` Binary Utility

This utility is used to move files from one directory to another. It is useful when you have a large number of files to move and you want to do it quickly.

## Usage


Your possible directory file structure:

```txt
some_dir/
  notes/
    2024-10-08.txt
    2024-08-07.txt
    2024-10-08.txt

    2023-12-11.txt

    notes-2024/
      ...
```
Use such command to move all files starts with `2024` from `notes` directory to `notes-2024` directory:

```bash
./mmv './notes/2024-*.txt' './notes/notes-2024/2024-#1.txt'
```

After that, you will have:

```txt
some_dir/
  notes/
    2023-12-11.txt

    notes-2024/
      2024-10-08.txt
      2024-08-07.txt
      2024-10-08.txt
```

### Testing
```bash
cargo test
```

### Clippy Linting
```bash
cargo clippy
```
