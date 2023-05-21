## gm_file
Allows you to manipulate files in the GarrysMod/garrysmod directory

# Use
1. Add a dependency to your project in the Cargo.toml file
```toml
[dependencies]
gm_file = "*"
```
2. Add this in main code file
```rust
use gm_file::*;
```

# Functions

```rust
pub fn create_file(name: &str, content: &str) -> std::io::Result<()> // creates file
pub fn remove_file(name: &str) -> std::io::Result<()> // removes file
```

# Example

```rust
let name: &str = "lua/autorun/cl_minono.lua";
let content: &str = "local function Trash() print('Rust is tha best language ever') end";
create_file(name, content); // creates file "cl_minono.lua" in "GarrysMod/garrysmod/lua/autorun"
```
