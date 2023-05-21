# gm_file
Allows you to manipulate files in the GarrysMod/garrysmod directory

### Use
1. Add a dependency to your project in the Cargo.toml file
```toml
[dependencies]
gm_file = "*"
```
2. Add this in main code file
```rust
use gm_file::*;
```

### Functions

```rust
pub fn create_file(name: &str, content: &str) -> std::io::Result<()> // creates file
pub fn remove_file(name: &str) -> std::io::Result<()> // removes file
```
