# Commands

These are commands I use on a regular basis for developement.

## Running tests

Change model_db_ the file name in `src/_tests/` directory

```bash
cd backend
cargo watch -q -c -w src/ -x 'test model_db_ -- --test-threads=1 --nocapture'
```
