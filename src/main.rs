use rand::Rng;

struct Nazar {
    name: String,
    age: i32,
    fumeaza: Option<bool>
}

impl Nazar {
    pub fn new(name:String, age: i32) -> Self {
        Self {
            name,
            age,
            fumeaza: None
        }
    }
    pub fn increment_age(&mut self) {
        self.age = self.age +1;
    }
}

fn main() {
    static MY_VAR: &str = "";
    
    let mut rng = rand::thread_rng();
    let y: i32 = rng.gen(); // generates a float between 0 and 1

    let mut nazar = Nazar::new("Nazar".to_string(), 18);

    nazar.increment_age();

    println!("{:?}",nazar.age);


    let x: f64;

    // Option
    // Result

    let opt = Some(5);
    let opt_2: Option<i32> = None;
    
    println!("Hello, {:?}!", y);
}
