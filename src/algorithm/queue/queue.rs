#[derive(Debug)]

pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {


     pub fn new() -> Self {
        Queue { items: Vec::new()}
     }

     pub fn enqueue(&mut self, item: T){
         self.items.push(item);
     }
     
     pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty(){
            None
        }else{
            Some(self.items.remove(0))
        }
     }

     pub fn front(&self) -> Option<&T> {
        self.items.first()
     }

     pub fn rear(&self) -> Option<&T> {
        self.items.last()
     }

     pub fn is_empty(&self) -> bool {
        self.items.is_empty()
     }

     pub fn size(&self) -> usize {
        self.items.len()
     }

     pub fn clear(&mut self) {
        self.items.clear()
     }

     pub fn to_vec(&self) -> Vec<T> where T: Clone {
        self.items.clone()
     }

     pub fn from_vec(items: Vec<T>) -> Self {
        Queue { items }
     }

     pub fn peak_at(&self, index: usize) -> Option<&T> {
        self.items.get(index)
     }

     pub fn display(&self) 
     where T: std::fmt::Display
     {
         if self.is_empty() {
            println!("Queue is empty");
            return;
        }

        print!("Queue (front to rear): ");
        for (index, item) in self.items.iter().enumerate() {
            if index == 0 {
                print!("FRONT[{}]", item);
            } else if index == self.items.len() - 1 {
                print!(" -> [{}]REAR", item);
            } else {
                print!(" -> [{}]", item);
            }
        }
        println!();

     }  

}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}