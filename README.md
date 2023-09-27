# minigrep

"minigrep" utility [described in Rust book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

---

### Usage

```shell
IGNORE_CASE=0 cargo run -- "query_string" /path/to/file.txt 
```

---

### TODO

- Highlight matched substrings in lines if output shown in terminal.
  Control this behavior with an env variable (e.g. "HIGHLIGHT_MATCHES=1").
