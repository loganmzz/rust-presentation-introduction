## Ownership

* Ownership `Move`, `Copy`
* Borrowing (`&`)
* Timelife

---

```java
class Cache<K,V> implements {

  Map<K, CacheEntry<V>> index = new HashMap<>();

  void evictExpired() {
    for (Entry<K, CacheEntry<V>> entry : index.entrySet()) {
      if (index.getValue().hasExpired()) {
        content.remove(entry.getKey());
        // ConcurrentModificationException
        //   à l'itération suivante
      }
    }
  }
}
```