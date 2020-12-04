struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
} 


fn main() {
    // let p = Point { x: 100, y: 200 };
    // println!("{} {}", p.x, p.y);
    // let x = 100;
    // println!("{}", x);

    let n = 5;

    for i in 0..10 {
        println!("{}", i);
        if i == n {
            println!("n == {}", n)
        }
    }

    let r = Rect { width: 300, height: 200 };
    let a = r.area();

    println!("{}", a)
}

