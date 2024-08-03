pub fn simplify_path(path: &str) -> String {
        let mut stack = vec![];
        for s in path.split('/').filter(|s| !s.is_empty()) {
            match s {
                "." => (),
                ".." => {
                    stack.pop();
                }
                _ => stack.push(s),
            }
        }
        format!("/{}", stack.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(simplify_path("/home/"), "/home");
        debug_assert_eq!(simplify_path("/home//foo/"), "/home/foo");
        debug_assert_eq!(
            simplify_path("/home/user/Documents/../Pictures"),
            "/home/user/Pictures"
        );
        debug_assert_eq!(simplify_path("/../"), "/");
        debug_assert_eq!(simplify_path("/.../a/../b/c/../d/./"), "/.../b/d");
    }

    #[test]
    fn test() {}
}
