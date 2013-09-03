use intrusive_list::*;
use std::hashmap::*;

struct LRUEntry<K, V> {
  key: K,
  value: V,
  node: IntrusiveNode<LRUEntry<K, V>>
}

struct LRUCache<K, V, E> {
  map: HashMap<K, @mut LRUEntry<K, V>>,
  list: IntrusiveList<LRUEntry<K, V>>,
  size: uint,
  max_size: uint,
  lookup: @fn(&K) -> Result<V, E>
}

fn access_node<'r, K, V>(entry: &'r mut LRUEntry<K, V>) -> &'r mut IntrusiveNode<LRUEntry<K, V>> {
  &mut (*entry).node
}

impl<K: Hash + Eq + 'static + Clone, V: Clone + 'static, E> LRUCache<K, V, E> {
  pub fn new(lookup: @fn(&K) -> Result<V, E>, max_size: uint) -> LRUCache<K, V, E> {
    LRUCache {
      map: HashMap::new(),
      list: IntrusiveList::new(access_node),
      size: 0,
      max_size: max_size,
      lookup: lookup
    }
  }

  pub fn lookup(&mut self, key: K) -> Result<V, E> {
    match self.map.find(&key) {
      Some(value) => {
        self.list.push_front(*value);
        return Ok(value.value.clone());
      },
      None => {}
    };

    match (self.lookup)(&key) {
      Ok(value) => {
        let entry = @mut LRUEntry { key: key.clone(), value: value.clone(), node: IntrusiveNode::new() };
        self.list.push_front(entry);
        self.map.insert(key, entry);
        self.size += 1;
        if (self.size > self.max_size) {
          match self.list.pop_back() {
            Some(val) => {
              self.map.remove(&val.key);
              self.size -= 1;
            },
            None => {
              println("impossible");
            }
          }
        }
        Ok(value)
      },
      Err(err) => Err(err)
    }
  }
}
