mod message;

// Add this at the top of the file
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub use message::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Messages<M> {
    list: Vec<Message<M>>,
    selected: usize,
    pub id: u64,

    /// Can we go forward?
    pub forward: bool,

    /// Can we go backward?
    pub backward: bool,
}

impl<M> Messages<M> {
    pub fn new(message: Message<M>, id: Option<impl Into<u64>>) -> Self {
        Self {
            list: vec![message],
            id: id.map_or(rand::random(), |id| id.into()),
            backward: false,
            forward: false,
            selected: 0,
        }
    }

    /// Pushes and selects the message
    pub fn push(&mut self, message: impl Into<Message<M>>) {
        self.list.push(message.into());

        self.selected = self.list.len() - 1;

        self.forward = self.selected + 1 < self.list.len();
        self.backward = self.selected > 0;
    }
    /// Pushes and returns the message index without selecting it
    pub fn push_silent(&mut self, message: impl Into<Message<M>>) -> usize {
        self.list.push(message.into());

        self.forward = self.selected + 1 < self.list.len();
        self.backward = self.selected > 0;

        self.list.len() - 1
    }

    /// O(1) amortized
    pub fn skip_to(&mut self, index: usize) -> Option<&M> {
        match self.list.get(index) {
            Some(message) => {
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
        self.select(self.selected).unwrap()
    }
    /// O(1) constant
    pub fn select(&self, index: usize) -> Option<&M> {
        self.list.get(index).map(|m| &m.message)
    }
    /// O(1) constant
    pub fn mut_selected(&mut self) -> &mut M {
        self.mut_select(self.selected).unwrap()
    }
    /// O(1) constant
    pub fn mut_select(&mut self, index: usize) -> Option<&mut M> {
        self.list.get_mut(index).map(|m| &mut m.message)
    }
    /// O(1) amortized
    pub fn into_select(mut self, index: usize) -> Option<M> {
        if index >= self.list.len() {
            return None;
        }
        let message = self.list.swap_remove(index);
        Some(message.message)
    }
    /// O(1) amortized
    pub fn into_selected(mut self) -> M {
        self.list.swap_remove(self.selected).message
    }
}
