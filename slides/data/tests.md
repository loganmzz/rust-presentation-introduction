<!-- .slide: data-background="/assets/img/why_tests_are_important.gif" -->

---

## Unit tests

``` rust
#[test]
fn worker_thread_index() {
    let pool = ThreadPool::new(Configuration::new()
                            .num_threads(8)).unwrap();
    
    assert_eq!(pool.current_num_threads(), 8);
    assert_eq!(pool.current_thread_index(), None);
    
    let index = pool.install(|| 
        pool.current_thread_index().unwrap()
    );
    
    assert!(index < 22);
}
```

[rayon-core/src/test.rs](https://github.com/nikomatsakis/rayon/blob/master/rayon-core/src/test.rs)

---

## Writing tests
* Add `#[test]` attribute

* `assert!` macros provide in std

* `#[ignore]`, `#[should_panic]`, ...

* Place in the `src/*`

---

## Integration tests

* tests/*.rs

* different `binaries` for each tests

---

## Documentation tests

<pre>
<code class="rust" data-trim data-noescape>
/// Append a name and value for the `Cookie`.
///
/// ```
/// use hyper::header::Cookie;
/// let mut cookie = Cookie::new();
///
/// cookie.append("foo", "bar");
/// cookie.append("foo", "quux");
///
/// assert_eq!(cookie.to_string(), "foo=bar; foo=quux");
/// ```
pub fn append<K, V>(&mut self, key: K, value: V) where K: Into<Cow<'static, str>>, V: Into<Cow<'static, str>>
{
    self.0.append(key.into(), value.into());
}
</code>
</pre>

[src/header/common/cookie.rs](https://github.com/hyperium/hyper/blob/1059eb349a560a4b9b83181acd9db19d1ef42073/src/header/common/cookie.rs)

---

## Concurrency

By `default` tests run `concurrently`

``` bash
$ RUST_TEST_THREADS=5 cargo test   # Run tests with concurrency

...

$ cargo test -- --test-threads=5   # Same as above
...
```