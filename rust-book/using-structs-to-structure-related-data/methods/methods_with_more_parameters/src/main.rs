#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn check_if_it_fits_inside(
        &self, 
        rectangle_to_check: &Rectangle
    ) -> bool {        
        #[cfg(debug_assertions)]
        {
            dbg!(self);
            dbg!(&rectangle_to_check);
    
            dbg!(
                self.width >= rectangle_to_check.width 
                 && 
                self.height >= rectangle_to_check.height
            )
        }

        #[cfg(not(debug_assertions))]
        {
            self.width >= rectangle_to_check.width 
             && 
            self.height >= rectangle_to_check.height
        }
    }
}

fn main() {
    let rectangle_01 = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle_02 = Rectangle {
        width: 10,
        height: 40,
    };

    let rectangle_03 = Rectangle {
        width: 60,
        height: 45
    };

    println!(
        "Can 'rectangle_01' keep inside 'rectangle_02': {}\n",
        rectangle_01.check_if_it_fits_inside(
            &rectangle_02
        )
    );

    println!(
        "Can 'rectangle_01' keep inside 'rectangle_03': {}\n",
        rectangle_01.check_if_it_fits_inside(
            &rectangle_03
        )
    )
}
