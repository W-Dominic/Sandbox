#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32, 
}

fn main() {
   let rectangle = Rect {
       height: 50,
       width: 30,
   };
   //pass by ref to stay in scope
   let area: u32 = calc_area(&rectangle); 
   //different ways to print a struct
   println!("Our rectangle: {:?}", rectangle);
   println!("Our rectangle: {:#?}", rectangle);
   dbg!(&rectangle);
   //our result
   println!("The area is: {} square units", area);
}

fn calc_area(rectangle: &Rect) -> u32 {
    rectangle.width * rectangle.height
}
