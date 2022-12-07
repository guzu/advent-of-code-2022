use std::os::unix::fs;

fn main() {
    let _e = fs::symlink("../../scripts/pre-push", 
                         ".git/hooks/pre-push");
}
