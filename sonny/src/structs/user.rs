use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    // probably just base with a couple formatting changes?
}

#[derive(Serialize, Deserialize)]
struct Member {
    // should essentially be user obj + guild obj 
}