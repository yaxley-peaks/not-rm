## Delete all files that **Do Not** match the given extention

An excessively simple tool I wrote to manage all the garbage .o and .hi files
GHC generates. If you are here, may you find this useful

## Usage
```bash
not-rm <INPUT> <EXT>
    where
        INPUT: Directory to look in.
        EXT: File extention to NOT delete. Please do not include the (.)
```
## Example
`not-rm ./target .o`


### TODOs
- [x] Multiple extensions together
- [x] Use rust's native extension matching
- [x] Standardize pattern syntax (include .?)
- [x] Write better documentation with clap