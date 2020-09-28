impl<K, V> ConcurrentMap<K, V> {
    fn insert(&self, K, V) -> Result<(), (K, V)>;
    fn get(&self, &K) -> Option<&V>;
    fn remove(&self, &K) -> Option<(K, V)>
}

impl<T> Promise<T> {
    fn fulfill(self, T);
}

impl<T> Future<T> {
    fn
}
