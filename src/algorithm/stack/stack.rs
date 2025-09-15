#[derive(Debug)]

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {

   pub fn new() -> Self {
    Stack {
        items: Vec::new(),
    }
   }

   pub fn push(&mut self, item: T){
     self.items.push(item);
   }

    pub fn pop(&mut self) -> Option<T> {
      self.items.pop()
   }

   pub fn peek(&mut self) -> Option<&T> {
    self.items.last()
   }

   pub fn is_empty(&self) -> bool {
     self.items.is_empty()
   }

   pub fn size(&self) -> usize {
     self.items.len()
   }

   pub fn clear(&mut self) {
     self.items.clear(); 
   }

   pub fn display(&self) where T: std::fmt::Display {

        if self.is_empty() {
            println!("Stack is empty");
            return;
        }

        println!("Stack elements:");
        for (index, item) in self.items.iter().enumerate() {
            if index == self.items.len() - 1 {
                println!("  {} <- TOP", item);
            } else {
                println!("  {}", item);
            }
        }

   }

   pub fn to_vec(&self) -> Vec<T> where T: Clone {
     self.items.clone()
   }

   pub fn from_vec(vec: Vec<T>) -> Self {
      Stack {
        items: vec,
      }
   } 
 
}


impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}