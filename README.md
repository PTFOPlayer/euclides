# euc_lib
### Liblary implements:
 * euc - euclidean algorithm
 * euc_ext - extended euclidean algorithm
 * lcm - least common multiple
 * congruence - congruence soliving function, finding smallest x for solution

# Want to contribute?:
### My github:
<a href="https://github.com/PTFOPlayer">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@3.0.1/icons/github.svg" height="45px" alt="github" />
</a>

### Project:
<a href="https://github.com/PTFOPlayer/euclides">
<img src="https://cdn.jsdelivr.net/npm/simple-icons@3.0.1/icons/github.svg" height="45px" alt="github" />
</a>

### Support:
<p><a href="https://www.buymeacoffee.com/WhiskyAKM"> <img align="left" src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" height="50" width="210" alt="https://www.buymeacoffee.com/WhiskyAKM" /></a></p>
<br/>
<br/>

# Example usage:
## Euclides
### Extended
> Program
```rs
use euc_lib;
fn main() {
    prinln!("{}", euc_lib::I32::euc_ext(135, 35));
}
```
> Output
```
NWD = 5, S = -1, T = 4
```
### Simple
> Program
```rs
use euc_lib;
fn main() {
    prinln!("{}", euc_lib::I32::euc(135, 35)); // there is recursive variant too: euc_recursive(135,35)
}
```
> Output
```
5
```
### Vector as an argument
> Program
```rs
use euc_lib;
fn main() {
    println!("{:?}", euc_lib::I32::euc_from_vec(vec![21, 14, 56]));
}
```
> Output
```
Ok(7)
```

## LCM
### Simple
This version implements Least common multiple calculating method using gcd (Euclidean algorithm)
> Program
```rs
use euc_lib;
fn main () {
    println!("{}", euc_lib::I32::lcm(21, 6)) // there is recursive variant too: lcm_recursive
}
```
> Output
```
42
```

### Vector as argument
> Program
```rs
use euc_lib;
fn main() {
    println!("{:?}", euc_lib::I32::lcm_from_vec(vec![12,4,8]))
}
```
> Output
```
Ok(24)
```

## Congruence 
> Program
```rs
use euc_lib;
fn main() {
    println!("{:?}", euc_lib::I32::congruence(9,21,30))
}
```
> Output
```
Ok(9)
```
## i64 support
To use i64 versions of all functions just use euc_lib::I64 instead of euc_lib::I32
### Example of I64 usage
> Program
```rs
use euc_lib;
fn main() {
    prinln!("{}", euc_lib::I64::euc_ext(135, 35));
}
```
> Output
```
NWD = 5, S = -1, T = 4
```