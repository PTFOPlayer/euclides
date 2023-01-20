# euclides
Simple implementation of euclides algorithm

## Example usage:
### Extended
> Program
```
use euc_lib;
fn main() {
    prinln!("{:?}, euc_lib::euc_ext(135, 35)");
}
```
> Output
```
EucRes { d: 5, s: -1, t: 4 }
```
### Simple
> Program
```
use euc_lib;
fn main() {
    prinln!("{}, euc_lib::euc(135, 35)");
}
```
> Output
```
5
```