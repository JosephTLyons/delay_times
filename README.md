# delay_times

Calculate delay times, in milliseconds and in hertz, for a given tempo

```rust
use delay_times::DelayTimes;

fn main() {
    let delay_times = DelayTimes::in_ms(120.0);

    println!("{}", delay_times.v_quarter);

    // 500
}
```
