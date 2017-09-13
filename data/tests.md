<!-- .slide: data-background="assets/img/productive.gif" -->

---

<!-- .slide: data-background="assets/img/why_tests_are_important.gif" -->

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

## Unit tests Recipie

1. Place in the `src/*`

2. Add `#[test]` attribute (`#[ignore]`, `#[should_panic]` ...)

3. Use `assert!` macros

4. Run `$cargo test`

---

## Integration tests

* tests/*.rs

* different _binaries_ for each tests

---

## Documentation tests

````rust
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
pub fn append<K, V>(&mut self, key: K, value: V) where
    K: Into<Cow<'static, str>>,
    V: Into<Cow<'static, str>>
{
    self.0.append(key.into(), value.into());
}
````

[src/header/common/cookie.rs](https://github.com/hyperium/hyper/blob/1059eb349a560a4b9b83181acd9db19d1ef42073/src/header/common/cookie.rs)

---

## Concurrency

By _default_ tests run _concurrently_

<pre><code class="bash" data-trim data-noescape>
$ <span class="fragment highlight-mark">RUST_TEST_THREADS=5</span> cargo test   # Run tests with concurrency

...

$ cargo test <span class="fragment highlight-mark">-- --test-threads=5</span>   # Same as above
</pre></code>

---

## Test frameworks

[e.g. stainless](https://github.com/reem/stainless)

``` rust
    describe! addition {
        before_each {
            let x = 5;
            let y = 6;
        }

        it "should add 5 and 6 together" {
            assert_eq!(x + y, 11);
        }

        after_each {
            assert_eq!(x, 5);
            assert_eq!(y, 6);
        }
    }
```
