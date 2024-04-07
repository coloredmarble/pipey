
this is a relatively shitty clone of [`tap`](https://crates.io/crates/tap) but in 3 lines (for readability)

made this because i am bored as fuck

```
tap = access the value temporarily. and return it
pipe = access value and return whatever
poof = like "pipe". marks the fact that the value will be dropped (all returns are ignored). this allows chaining multiple statements
```

usage:

```
use pipey::Pipey;
fn main(){
    // the value is created
    (10)
    // this will allow temporary access to the value
    .tap(|n| println!("value before adding 10 to it is {n}")).btw(|| println!("the earth is flat!")) // do something unrelated to the result
    // this will create another value. using the previous one
    .pipe(|n| n + 10)
    // the next function call will be the last to use this value!
    .poof(|n| println!("the value is {n}"));
    // the value is dropped. not leaving a mark, you will be the last to remember it, and then it will be gone again, forever
}
```