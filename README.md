# PipeViewer
This is a simplified rust version of the [pv](https://man7.org/linux/man-pages/man1/pv.1.html) utility.

## Display Features
- Total bytes read
- Time elapsed
- Bytes/sec

## Example usage
```
-o --output == where to write the output
-s --silent == will not display statistics in terminal
```
```
yes | cargo run -- -o dev/null
```



