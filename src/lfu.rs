
pub mod LFU {
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
#[derive(Debug)]
pub struct LFUCache {
    len: i32,
    cap: i32,
    keys: HashMap<i32, Rc<RefCell<Node>>>,
    freqs: HashMap<i32, Rc<RefCell<Freq>>>,
    head: Option<Rc<RefCell<Freq>>>,
}
#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    freq: i32,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
    parent: Option<Weak<RefCell<Freq>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key: key,
            val: value,
            freq: 1,
            next: None,
            prev: None,
            parent: None,
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

#[derive(Debug)]
struct Freq {
    f: i32,
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Freq>>>,
    prev: Option<Weak<RefCell<Freq>>>,
}

impl Freq {
    fn new(frequency: i32) -> Self {
        Freq {
            f: frequency,
            head: None,
            tail: None,
            next: None,
            prev: None,
        }
    }
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        let freq = Rc::new(RefCell::new(Freq::new(1)));
        let mut f: HashMap<i32, Rc<RefCell<Freq>>> = HashMap::new();
        f.insert(1, freq.clone());
        LFUCache {
            head: Some(freq.clone()),
            keys: HashMap::new(),
            freqs: f,
            cap: capacity,
            len: 0,
        }
    }

    fn move_node(&mut self, node: Rc<RefCell<Node>>) {
        node.borrow_mut().freq += 1;

        if node
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .next
            .is_none()
        {
            /*
            ///
            ///
            ///                 IF THE NODE DOESNT HAVE NEXT FREQUENCY NODE
            ///
            ///
            ///
            /// */

            /* CREATE NEW PARENT */
            let next_parent = Rc::new(RefCell::new(Freq::new(node.borrow().freq)));
            self.freqs.insert(node.borrow().freq, next_parent.clone());

            /* SET THE REFERENCE TO A NEW PARENT NODE FROM PREVIOUS PARENT */
            node.borrow_mut()
                .parent
                .as_mut()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow_mut()
                .next = Some(next_parent.clone());

            /*  MAKE NEW PARENT POINT TO THE PREVIOUS PARENT  */
            next_parent.borrow_mut().prev = Some(Rc::downgrade(
                &node
                    .borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .clone(),
            ));

            /*   CONFIGURE NEW HEAD AND TAIL FOR NEW PARENT   */
            next_parent.borrow_mut().head = Some(node.clone());
            next_parent.borrow_mut().tail = Some(node.clone());

            /*   ATTACH NODE TO A NEW PARENT   */
            node.borrow_mut().parent = Some(Rc::downgrade(&next_parent.clone()));
        }
        /*
                         BY THIS TIME WE SURE WE HAVE NEXT PARENT NODE.
                        NEXT THING WE NEED TO CHECK, WHETHER NEXT FREQUENCY NODE HAS THE SAME
                        FREQUENCY AS OUR NODE
        */

        if node
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .next
            .is_some()
            && node.borrow().freq
                < node
                    .borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .f
        {
            /*
            ///
            ///
            ///
            ///                 IF THE NEXT PARENT NODE FREQUENCY GREATER THAN NODE NEW FREQUENCY
            ///
            ///
            ///
            /// */

            /*  WE CREATE NEW PARENT NODE TO INSERT BETWEEN NODE PARENT AND NEXT PARENT */
            let new_parent = Rc::new(RefCell::new(Freq::new(node.borrow().freq)));
            self.freqs.insert(node.borrow().freq, new_parent.clone());

            /* MAKE NEXT FREQUENCY NODE TO POINT TO OUR NEW PARENT NODE */
            node.borrow()
                .parent
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow_mut()
                .prev = Some(Rc::downgrade(&new_parent.clone()));

            /*   MAKE OUR NEW PARENT POINT TO THE NEXT FREQUENCY NODE  */
            new_parent.borrow_mut().next = Some(
                node.borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap()
                    .clone(),
            );

            /*  MAKE OUR NEW PARENT TO POINT TO THE OLD ONE  */
            new_parent.borrow_mut().prev = Some(Rc::downgrade(
                &node
                    .borrow()
                    .parent
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .clone(),
            ));

            /*   MAKE OUR CURRENT PARENT POINT TO THE NEW PARENT   */
            node.borrow_mut()
                .parent
                .as_mut()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow_mut()
                .next = Some(new_parent.clone());

            /*   NEW HEAD AND TAIL   */
            new_parent.borrow_mut().head = Some(node.clone());
            new_parent.borrow_mut().tail = Some(node.clone());

            /*   ATTACH NODE TO A NEW PARENT   */
            node.borrow_mut().parent = Some(Rc::downgrade(&new_parent.clone()));
        }

        /*   CHECK IF WE GOT APPROPRIATE PARENT   */
        if node
            .borrow()
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .borrow()
            .f
            != node.borrow().freq
        {
            let parent = self.freqs.get(&node.borrow().freq).unwrap();

            parent.borrow_mut().tail.as_mut().unwrap().borrow_mut().next = Some(node.clone());

            node.borrow_mut().parent = Some(Rc::downgrade(&parent.clone()));
        }

        /*

                    BY THIS TIME WE SURE WE HAVE PARENT FREQUENCY NODE WITH APPROPRIATE FREQUENCY
        */

        /* CONSIDER 4 CASES */

        /*  1. OUR NODE HAS BOTH PREVIOUS AND NEXT NODES  */
        if node.borrow().prev.is_some() && node.borrow().next.is_some() {
            let next_mut = node.borrow_mut().next.as_mut().unwrap().clone();
            let next = node.borrow().next.as_ref().unwrap().clone();

            let prev_mut = node
                .borrow_mut()
                .prev
                .as_mut()
                .unwrap()
                .upgrade()
                .unwrap()
                .clone();
            let prev = node
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .clone();

            prev_mut.borrow_mut().next = Some(next);
            next_mut.borrow_mut().prev = Some(Rc::downgrade(&prev));

            node.borrow_mut().prev = None;
            node.borrow_mut().next = None;

        /*  2. OUR NODE HAS ONLY PREVIOUS NODE  */
        } else if node.borrow().prev.is_some() {
            let parent = self.freqs.get(&(node.borrow().freq - 1)).unwrap();
            parent.borrow_mut().tail = Some(
                node.borrow()
                    .prev
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .clone(),
            );

            /*     AND GET RID OF THE PREVIOUS NODE POINTER     */
            node.borrow_mut()
                .prev
                .as_mut()
                .unwrap()
                .upgrade()
                .unwrap()
                .borrow_mut()
                .next = None;
            node.borrow_mut().prev = None;
        } else if node.borrow().next.is_some() {
            /*  3. OUR NODE HAS ONLY NEXT NODE  */
            let parent = self.freqs.get(&(node.borrow().freq - 1)).unwrap();
            parent.borrow_mut().head = Some(node.borrow().next.as_ref().unwrap().clone());
            node.borrow_mut().next.as_mut().unwrap().borrow_mut().prev = None;
            node.borrow_mut().next = None;
        } else {
            /*  4. OUR NODE HAS NEITHER PREVIOUS NOR NEXT NODES  */

            /*THIS MEANS IT CAN BE BOTH HEAD AND TAIL*/
            if *self
                .head
                .as_ref()
                .unwrap()
                .borrow()
                .head
                .as_ref()
                .unwrap()
                .borrow()
                == *node.borrow()
            {
                /*    THEN WE MOVE THE ENTIRE HEAD TO THE NEW PARENT  */
                self.head = Some(
                    node.borrow()
                        .parent
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .clone(),
                );
            }
            let old_parent = self.freqs.get_mut(&(node.borrow().freq - 1)).unwrap();
            if old_parent.borrow().prev.is_some()
                && old_parent
                    .borrow()
                    .prev
                    .as_ref()
                    .unwrap()
                    .upgrade()
                    .is_some()
            {
                node.borrow_mut()
                    .parent
                    .as_mut()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .prev = Some(Rc::downgrade(
                    &old_parent
                        .borrow()
                        .prev
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .clone(),
                ));
                old_parent
                    .borrow_mut()
                    .prev
                    .as_mut()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .next = Some(
                    node.borrow()
                        .parent
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .clone(),
                );
            }
            self.freqs.remove(&(node.borrow().freq - 1));
        }

        /*
        now handle inner connections
         */
        let parent = self.freqs.get_mut(&node.borrow().freq).unwrap();
        if *parent.borrow().tail.as_ref().unwrap().borrow() != *node.borrow() {
            node.borrow_mut().prev = Some(Rc::downgrade(
                &parent.borrow().tail.as_ref().unwrap().clone(),
            ));
            parent.borrow_mut().tail = Some(node.clone());
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.keys.remove(&key) {
            Some(node) => {
                self.move_node(node.clone());
                self.keys.insert(key, node.clone());
                node.borrow().val
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.cap == 0 {
            return;
        }
        match self.keys.remove(&key) {
            Some(node) => {
                node.borrow_mut().val = value;
                self.move_node(node.clone());
                self.keys.insert(key, node.clone());
            }
            None => {
                let node = Rc::new(RefCell::new(Node::new(key, value)));
                if self.len == self.cap {
                    self.invalidate()
                } else {
                    self.len += 1
                }
                self.add_new(node.clone());
                self.keys.insert(key, node);
            }
        }
    }

    fn invalidate(&mut self) {

        self.keys.remove(
            &self
                .head
                .as_ref()
                .unwrap()
                .borrow()
                .head
                .as_ref()
                .unwrap()
                .borrow()
                .key,
        );

        //if deleting node has a child, we move pointer to the next node and delete pointer to deleting node
        if self
            .head
            .as_ref()
            .unwrap()
            .borrow()
            .head
            .as_ref()
            .unwrap()
            .borrow()
            .next
            .is_some()
        {
            let next_node = self
                .head
                .as_ref()
                .unwrap()
                .borrow()
                .head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .clone();
            self.head.as_mut().unwrap().borrow_mut().head = Some(next_node);
            self.head
                .as_mut()
                .unwrap()
                .borrow_mut()
                .head
                .as_mut()
                .unwrap()
                .borrow_mut()
                .prev = None;
            return;
        }
        let removed = self
            .freqs
            .remove(&self.head.as_ref().unwrap().borrow().f)
            .unwrap();

        if removed.borrow().next.is_some()
            && removed.borrow().prev.is_some()
            && removed.borrow().prev.as_ref().unwrap().upgrade().is_some()
        {
            let prev_mut = removed
                .borrow_mut()
                .prev
                .as_mut()
                .unwrap()
                .upgrade()
                .unwrap()
                .clone();
            let prev = removed
                .borrow()
                .prev
                .as_ref()
                .unwrap()
                .upgrade()
                .unwrap()
                .clone();

            let next_mut = removed.borrow_mut().next.as_mut().unwrap().clone();
            let next = removed.borrow().next.as_ref().unwrap().clone();

            prev_mut.borrow_mut().next = Some(next);
            next_mut.borrow_mut().prev = Some(Rc::downgrade(&prev));
        } else if removed.borrow().next.is_some() {
            let next_head = self.head.as_ref().unwrap().clone();
            self.head = Some(next_head.borrow().next.as_ref().unwrap().clone());
            self.head.as_mut().unwrap().borrow_mut().prev = None;
        } else {
            let freq = Rc::new(RefCell::new(Freq::new(1)));
            self.head = Some(freq.clone());
            self.freqs.insert(1, freq);
        }
    }

    fn add_new(&mut self, node: Rc<RefCell<Node>>) {
        if self.head.as_ref().unwrap().borrow().f != 1 {
            let freq_one = Rc::new(RefCell::new(Freq::new(1)));
            self.freqs.insert(1, freq_one.clone());
            freq_one.borrow_mut().next = Some(self.head.as_ref().unwrap().clone());
            self.head.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&freq_one));
            self.head = Some(freq_one.clone());
        }

        if self.head.as_ref().unwrap().borrow().tail.is_some() {
            self.head
                .as_mut()
                .unwrap()
                .borrow_mut()
                .tail
                .as_mut()
                .unwrap()
                .borrow_mut()
                .next = Some(node.clone());
            node.borrow_mut().prev = Some(Rc::downgrade(
                &self
                    .head
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .tail
                    .as_ref()
                    .unwrap()
                    .clone(),
            ));
        }
        node.borrow_mut().parent = Some(Rc::downgrade(&self.head.as_ref().unwrap().clone()));
        self.head.as_mut().unwrap().borrow_mut().tail = Some(node.clone());
        if self.head.as_ref().unwrap().borrow().head.is_none() {
            self.head.as_mut().unwrap().borrow_mut().head = Some(node)
        }
    }
}
}