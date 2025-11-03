pub struct Rectangle {
    widht: f64,
    height: f64
}


impl Rectangle {

    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width, height
        }
    }

    pub fn area(&self) -> f64 {
        self.width * self.height;
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

}