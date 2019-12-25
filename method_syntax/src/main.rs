struct Rectangle{
    width: f32,
    height: f32,
}

impl Rectangle{
    fn area(&self) -> f32{
        self.width + self.height
    }
    
    fn perimeter(&self) -> f32{
        2.0*(self.width + self.height)
    }
    
    fn square(size: f32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    
    fn is_golden_rectangle(&self) -> bool{
        self.width / self.height == 1.618
    }
}

fn main() {
    let rect1 = Rectangle{width: 64.72, height: 40.0};
    let rect2 = Rectangle::square(21.0);
    println!("A area do retangulo rect1 é {}", rect1.area());
    println!("A area do retangulo rect2 é {}", rect2.area());
    println!("O perimetro do retangulo rect1 é {}", rect1.perimeter());
    println!("O perimetro do retangulo rect2 é {}", rect2.perimeter());
    println!("rect1 pode sobrepor rect2 = {}", rect1.can_hold(&rect2));
    println!("rect2 pode sobrepor rect1 = {}", rect2.can_hold(&rect1));
    println!("rect1 é retangulo de ouro = {}", rect1.is_golden_rectangle());
    println!("rect2 é retangulo de ouro = {}", rect2.is_golden_rectangle());
}
