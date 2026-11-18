//! currently just testing the proc macro attribute
use sonny_pma::app_command;

#[app_command]
pub async fn test(test: i32) {
    println!("{}", test);
}
