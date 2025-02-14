mod message;

use indexmap::IndexMap;
pub use message::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Messages<M> {
    list: IndexMap<u64, Message<M>>,
    selected: usize,

    /// Can we go forward?
    pub forward: bool,

    /// Can we go backward?
    pub backward: bool,
}

impl<M> Messages<M> {
    pub fn new(message: Message<M>) -> Self {
        Self {
            list: IndexMap::from([(message.id, message)]),
            backward: false,
            forward: false,
            selected: 0,
        }
    }

    /// Pushes and selects the message
    pub fn push(&mut self, message: impl Into<Message<M>>) {
        let message = message.into();
        self.list.insert(message.id, message);

        self.selected = self.list.len() - 1;

        self.forward = self.selected + 1 < self.list.len();
        self.backward = self.selected > 0;
    }
    /// Pushes and returns the message index without selecting it
    pub fn push_silent(&mut self, message: impl Into<Message<M>>) -> usize {
        let message = message.into();
        self.list.insert(message.id, message);

        self.forward = self.selected + 1 < self.list.len();
        self.backward = self.selected > 0;

        self.list.len() - 1
    }

    /// O(1) amortized
    pub fn skip_to(&mut self, index: usize) -> Option<&M> {
        match self.list.get_index(index) {
            Some((_, message)) => {
                self.selected = index;

                self.forward = self.selected + 1 < self.list.len();
                self.backward = self.selected > 0;

                Some(&message.message)
            }
            None => None,
        }
    }
    /// O(1) constant (mirrors [`Self::selected`])
    /// Panics if no more messages to go forward (self.forward == false)
    pub fn forward(&mut self) -> &M {
        if !self.forward {
            panic!("PANIC! No more messages to go forward!");
        }

        self.selected += 1;

        self.forward = self.selected + 1 < self.list.len();
        self.backward = self.selected > 0;

        self.selected()
    }
    /// O(1) constant (mirrors [`Self::selected`])
    /// Panics if no more messages to go backward (self.backward == false)
    pub fn backward(&mut self) -> &M {
        if !self.backward {
            panic!("PANIC! No more messages to go backward!");
        }

        self.selected -= 1;

        self.forward = self.selected + 1 < self.list.len();
        self.backward = self.selected > 0;

        self.selected()
    }

    /// O(1) constant
    pub fn selected(&self) -> &M {
        self.get(self.selected).unwrap()
    }
    /// O(1) constant
    pub fn get(&self, index: usize) -> Option<&M> {
        self.list.get_index(index).map(|(_, m)| &m.message)
    }
    /// O(1) constant
    pub fn selected_mut(&mut self) -> &mut M {
        self.get_mut(self.selected).unwrap()
    }
    /// O(1) constant
    pub fn get_mut(&mut self, index: usize) -> Option<&mut M> {
        self.list.get_index_mut(index).map(|(_, m)| &mut m.message)
    }
    /// O(1) amortized
    pub fn get_into(mut self, index: usize) -> Option<M> {
        self.list.swap_remove_index(index).map(|(_, m)| m.message)
    }
    /// O(1) amortized
    pub fn into_selected(mut self) -> M {
        self.list
            .swap_remove_index(self.selected)
            .map(|(_, m)| m.message)
            .unwrap()
    }

    /// O(1) amortized
    pub fn find(&self, id: u64) -> Option<&M> {
        self.list.get(&id).map(|m| &m.message)
    }
    /// O(1) amortized
    pub fn find_mut(&mut self, id: u64) -> Option<&mut M> {
        self.list.get_mut(&id).map(|m| &mut m.message)
    }
    /// O(1) amortized
    pub fn find_into(mut self, id: u64) -> Option<M> {
        self.list.swap_remove(&id).map(|m| m.message)
    }
}
