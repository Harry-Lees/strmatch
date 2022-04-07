# strmatch

Python bindings for the [levenshtein](https://github.com/wooorm/levenshtein-rs) Rust library.
Used for calculating Levenshtein distance and ratio.

```python
def distance(n: str, m: str) -> int: ...
def ratio(n: str, m: str) -> float: ...
```

## Examples

Levenshtein distance

```python
>>> import strmatch
>>> str_one = 'test'
>>> str_two = 'tes'
>>> strmatch.distance(str_one, str_two)
1
```

Levenshtein Ratio

```python
>>> import strmatch
>>> str_one = 'test string'
>>> str_two = 'test STRING'
>>> strmatch.ratio(str_one, str_two)
0.4545454545454546
```
