# euc_lib
Easy to use implementation of extended and normanl Euclidean algorithm

## Example usage:
### Extended
> Program
```rs
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
```rs
use euc_lib;
fn main() {
    prinln!("{}, euc_lib::euc(135, 35)"); // theres is recursive variant too: euc_recursive(135,35)
}
```
> Output
```
5
```

## LCM 
This version implements Least common multiple calculating method using gcd (Euclidean algorithm)
> Program
```rs
use euc_lib;
fn main () {
    println!("{:?}", euc_lib::lcm(21, 6))
}
```
> Output
```
42
```

# Github:
### My github:
<a href="https://github.com/PTFOPlayer">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@3.0.1/icons/github.svg" height="45px" alt="github" />
</a>

### Project:
<a href="https://github.com/PTFOPlayer/euclides">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@3.0.1/icons/github.svg" height="45px" alt="github" />
</a>