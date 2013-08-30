struct IntrusiveNode<Node> {
  next: Option<Node>,
  prev: Option<Node>,
}

struct IntrusiveList<Node> {
  first: Option<Node>,
  last: Option<Node>,
  f: @fn(Node, &fn(&mut IntrusiveNode<Node>))
}

impl<Node: 'static> IntrusiveNode<Node> {
  fn new() -> IntrusiveNode<Node> {
    IntrusiveNode { next: None, prev: None }
  }
}

impl<Node: Clone> IntrusiveList<Node> {
  fn new(f: @fn(Node, &fn(&mut IntrusiveNode<Node>))) -> IntrusiveList<Node> {
    IntrusiveList { first: None, last: None, f: f }
  }

  fn push_front(&mut self, node: Node) {
    match self.first.clone() {
      Some(first) => {
        (self.f)(node.clone(), |node_node| {
          node_node.next = Some(first.clone());
        });
        (self.f)(first.clone(), |first_node| {
          first_node.prev = Some(node.clone());
        });
        self.first = Some(node.clone());
      }
      None => {
        self.first = Some(node.clone());
        self.last = Some(node.clone());
      }
    }
  }

  fn push_back(&mut self, node: Node) {
    match self.last.clone() {
      Some(last) => {
        (self.f)(node.clone(), |node_node| {
          node_node.prev = Some(last.clone());
        });
        (self.f)(last.clone(), |last_node| {
          last_node.next = Some(node.clone());
        });
        self.last = Some(node.clone());
      }
      None => {
        self.first = Some(node.clone());
        self.last = Some(node.clone());
      }
    }
  }

  fn pop_front(&mut self) -> Option<Node> {
    let first = self.first.clone();
    match first.clone() {
      Some(first_node) => {
        (self.f)(first_node.clone(), |node| {
          self.first = node.next.clone();
          match node.next {
            None => {
              self.last = None;
            },
            Some(_) => {}
          }
        })
      },
      None => {}
    }
    return first;
  }

  fn pop_back(&mut self) -> Option<Node> {
    let last = self.last.clone();
    match last.clone() {
      Some(last_node) => {
        (self.f)(last_node.clone(), |node| {
          self.last = node.prev.clone();
          match node.prev {
            None => {
              self.first = None;
            },
            Some(_) => {}
          }
        })
      },
      None => {}
    }
    return last;
  }
}

struct Foo {
  list_node: IntrusiveNode<@mut Foo>,
  n: int
}

fn print_list(list: &IntrusiveList<@mut Foo>) {
  print("[");
  match list.first {
    Some(first) => {
      print(fmt!("%d", first.n));
      let mut cur = first.list_node.next;
      loop {
        match(cur) {
          Some(current) => {
            print(fmt!(", %d", current.n));
            cur = current.list_node.next;
          },
          None => {
            print("]\n");
            return;
          }
        }
      }
    }
    None => {
      println("]");
    }
  }
}

fn main() {
  let a = @mut Foo { list_node: IntrusiveNode::new(), n: 42 };
  let b = @mut Foo { list_node: IntrusiveNode::new(), n: 24 };
  let get_list_node: @fn(@mut Foo, &fn(&mut IntrusiveNode<@mut Foo>)) = |node, f| {
    f(&mut (*node).list_node);
  };
  let mut list: IntrusiveList<@mut Foo> = IntrusiveList::new(get_list_node);

  print_list(&list);

  list.push_front(a);

  print_list(&list);

  list.push_back(b);

  print_list(&list);

  list.pop_front();

  print_list(&list);

  list.pop_front();

  print_list(&list);

  list.pop_front();

  print_list(&list);
}
