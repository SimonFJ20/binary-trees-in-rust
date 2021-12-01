

struct Stack <T>{
    index: usize,
    list: Vec<T>
}

impl<T> Stack <T>{
    
}

struct Node {
    value: i32,
    left: Option<*mut Node>,
    right: Option<*mut Node>
}

impl Node {
    pub fn print(&self, depth: usize) {
        println!("{}{}", " ".repeat(depth), self.value);
        unsafe {
            match self.left {
                Some(n) => (*n).print(depth + 1),
                None => {}
            }
            match self.right {
                Some(n) => (*n).print(depth + 1),
                None => {}
            }
        }
    }

    fn swap(&mut self) {
        let temp = self.left;
        self.left = self.right;
        self.right = temp;
    }
    
    pub fn revert_recursively(&mut self) {
        self.swap();
        unsafe {
            match self.left {
                Some(n) => (*n).revert_recursively(),
                None => {}
            }
            match self.right {
                Some(n) => (*n).revert_recursively(),
                None => {}
            }
        }
    }
    
    pub fn revert_stack(&mut self) {
        self.swap();
        unsafe {
            match self.left {
                Some(n) => (*n).revert_recursively(),
                None => {}
            }
            match self.right {
                Some(n) => (*n).revert_recursively(),
                None => {}
            }
        }
    }
}

fn main() {
    println!("Hello, world!");


    let mut tree = Node{
        value: 5,
        left: Some(&mut Node{
            value: 4,
            left: None,
            right: None
        }),
        right: Some(&mut Node{
            value: 6,
            left: Some(&mut Node{
                value: 5,
                left: None,
                right: None
            }),
            right: Some(&mut Node{
                value: 7,
                left: None,
                right: None
            })
        })
    };

    tree.print(0);
    tree.revert_recursively();
    tree.print(0);

}



