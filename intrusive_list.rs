use std::managed;

struct IntrusiveNode<Node> {
  next: Option<@mut Node>,
  prev: Option<@mut Node>,
}

struct IntrusiveList<Node> {
  first: Option<@mut Node>,
  last: Option<@mut Node>,
  f: @fn<'r>(&'r mut Node) -> &'r mut IntrusiveNode<Node>
}

impl<Node> IntrusiveNode<Node> {
  pub fn new() -> IntrusiveNode<Node> {
    IntrusiveNode { next: None, prev: None }
  }
}

impl<Node> IntrusiveList<Node> {
  pub fn new(f: @fn<'r>(&'r mut Node) -> &'r mut IntrusiveNode<Node>) -> IntrusiveList<Node> {
    IntrusiveList { first: None, last: None, f: f }
  }

  pub fn front(&mut self) -> Option<@mut Node> {
    self.first
  }

  pub fn last(&mut self) -> Option<@mut Node> {
    self.last
  }

  pub fn push_front(&mut self, node: @mut Node) {
    self.insert_before(node, self.last);
  }

  pub fn push_back(&mut self, node: @mut Node) {
    self.insert_after(node, self.first);
  }

  pub fn remove(&mut self, node: @mut Node) {
    let ptrs = (self.f)(node);

    match self.first {
      Some(first) => {
        if managed::mut_ptr_eq(first, node) {
          self.first = ptrs.next;
        }
      },
      None => {}
    }

    match self.last {
      Some(last) => {
        if managed::mut_ptr_eq(last, node) {
          self.last = ptrs.prev;
        }
      },
      None => {}
    }

    match ptrs.next {
      Some(next) => {
        (self.f)(next).prev = ptrs.prev;
        ptrs.next = None;
      },
      None => {}
    }

    match ptrs.prev {
      Some(prev) => {
        (self.f)(prev).next = ptrs.prev;
        ptrs.prev = None;
      },
      None => {}
    }
  }

  pub fn insert_after(&mut self, node: @mut Node, prev: Option<@mut Node>) {
    let next = self.next(prev);
    self.insert(node, prev, next);
  }

  pub fn insert_before(&mut self, node: @mut Node, next: Option<@mut Node>) {
    let prev = self.prev(next);
    self.insert(node, prev, next);
  }

  pub fn pop_back(&mut self) -> Option<@mut Node> {
    let last = self.last;
    match last {
      None => {},
      Some(l) => { self.remove(l); }
    };
    last
  }

  pub fn pop_front(&mut self) -> Option<@mut Node> {
    let first = self.first;
    match first {
      None => {},
      Some(f) => { self.remove(f); }
    };
    first
  }

  fn next(&mut self, node: Option<@mut Node>) -> Option<@mut Node> {
    match node {
      Some(n) => (self.f)(n).next,
      None => None
    }
  }

  fn prev(&mut self, node: Option<@mut Node>) -> Option<@mut Node> {
    match node {
      Some(n) => (self.f)(n).prev,
      None => None
    }
  }

  fn insert(&mut self, node: @mut Node, prev: Option<@mut Node>, next: Option<@mut Node>) {
    self.remove(node);

    match prev {
      None => {
        self.first = Some(node);
      },
      Some(p) => {
        (self.f)(p).next = Some(node);
        (self.f)(node).prev = prev;
      }
    }

    match next {
      None => {
        self.last = Some(node);
      },
      Some(n) => {
        (self.f)(n).prev = Some(node);
        (self.f)(node).next = next;
      }
    }
  }
}
