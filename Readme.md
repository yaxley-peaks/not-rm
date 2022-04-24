## Delete all files that **Do Not** match the given extention

An excessively simple tool i wrote to manage all the garbage .o and .hi files
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