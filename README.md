# Build
JSON formatted application build details provided by the Cargo.toml file.

---
## Usage

Add the following to your *Cargo.toml* file (Edit the version if needed.):
```
build = "*"
```

**Methods**  
  ***build_as_json()*** *->* *`String`*  
    Returns a JSON response formatted to the 1.1 specification that is in a String format for transit.

### Example
```
extern crate build;

fn main(){
  let details = build::build_as_json();
  println!("{}", details);
}
```
 To see it in action run (I pipe to `jq` to make it pretty!):
 ```
 cargo run | jq
 ```


## TODO
1. Encapsulate environment request with `try!` and alternate response structures when some cannot be obtained.
2. Provide string response for all `env` request types to *Cargo.toml* files offer.
3. ... [Suggestions?](https://github.com/webbrandon/build/labels/enhancement)
