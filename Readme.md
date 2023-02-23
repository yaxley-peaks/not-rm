## Delete all files that **Do Not** match the given extention

An excessively simple tool I wrote to manage all the garbage .o and .hi files
GHC generates. If you are here, may you find this useful

## Usage
```
not-rm <INPUT> <EXT>
    where
        INPUT: Directory to look in.
        EXT: File extention to NOT delete. Please include the (.)
```
## Example
`not-rm ./target .o`


### TODOs
- [ ] Multiple extensions together
- [ ] Use rust's native extension matching
- [ ] Standardize pattern syntax (include .?)