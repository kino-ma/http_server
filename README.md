# http_server
HTTP Server built with Rust.

## usage
```sh
$ cargo run <PAGE DIRECTORY>
```

### Example
```sh
$ cd http_server
$ cargo run pages
```

```sh
$ curl 'localhost:7878/hoge.html'
<!DOCTYPE html>
<html>
<head>
    <title>hogehoge</title>
</head>
<body>
    <h1>hogehoge</h1>
</body>
</html>
```
