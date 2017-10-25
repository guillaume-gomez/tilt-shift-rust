# tilt-shift-rust
I decided to learn Rust by using the library [Image](https://github.com/PistonDevelopers/image) and implement a [tilt-shift](https://en.wikipedia.org/wiki/Tilt%E2%80%93shift_photography) algorithm.

## Example 

### Original 
![original](tilt-shift/timeSquare.png)

### After tilt shift
![original](tilt-shift/timeSquareTiltShift.png)


## Compile 
`cargo build`

## Execute
```
cargo run  -- -b blur_level  -c staturation_level  -f filename_source -o filename_target -y originOfTheZoneOfTheInterest -h heightOfTheZoneOfTheInterest
#example
cargo run  -- -b 10.0  -c 20.0  -f "test.png" -o "result.png" -y 100 -h 50
```

## Help
`cargo run -- -h`
