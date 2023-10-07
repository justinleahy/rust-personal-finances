# Commands

These are commands I use on a regular basis for developement.

## Running tests

Runs all tests

```sh
cd backend
cargo watch -q -c -w src/ -x 'test _ -- --test-threads=1 --nocapture'
```

## Running web interface

```sh
cargo watch -q -c -w src/ -x 'run -- ../frontend/web-folder'
```
