# `fsserve`
__Working title...__  

Serves an fs structure as JSON.  
_Work in progress..._

## `fsserve-cli`
```
cargo run
```

## Example JSON response
```
$ tree --noreport .
.
└── root
    ├── bar
    ├── foo
    └── sub
        └── baz
$ fsserve-cli --port 8090 . &
$ curl localhost:8090/root
{
    name: "root",
    path: "/root/",
    type: "d",
    entries: [
        "/root/bar",
        "/root/foo",
        "/root/sub/"
    ]
}
```
