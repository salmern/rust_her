fn main() {
    println!("Hello, world!");


  let name = "binkabir".to_string(); 

  #[derive(Debug,Clone)]
  struct Person{
    name: String
  }
  

  let p1 = Person{name: "idris".to_string()};

  fn display_name(p: Person) -> String {
    format!("displaying person {:?}", p)
  }
  display_name( p1.clone()); 
  display_name(p1);

  let list = vec![2,3,5,4,3,6,8,9]; 
  let list_char = vec!['a', 'b', 'c', 'a', 'c', 'c']; 
  println!("found count {}", counter_int(&list, 3)); 
  println!("found char count {}", counter_char(&list_char, 'c')); 

  println!("using generic count found {}", generic_counter(&list, 3)); 


  ///traits 

  let m = Mercedes{ model: String::from("C300")}; 

  println!("{:?}", m.start_engine()); 


  let t = Toyota{ model: "Corolla".to_string()}; 

  println!("{:?}", t.dashboard()); 

  //traits as inputs; 
  println!("{:?}", car_display(&t)); 
  println!("{:?}", car_display(&m)); 



  //lifetime 

       let list2 = vec![5,6,77,2];
  

  println!("{:?}", longest_list(&list,&list2))
}


fn counter_int(list: &[i32], item: i32) -> i32 {
   let mut count = 0; 

   for i in list {
     if i == &item {
       count = count + 1 
     }
   }
  count 
}


fn counter_char(list: &[char], item: char) -> i32 {
   let mut count = 0; 

   for i in list {
     if i == &item {
       count = count + 1 
     }
   }
  count 
}


fn generic_counter<T: std::cmp::PartialEq>(list: &[T], item: T) -> i32 {
  let mut count: i32 = 0; 

  for i in list{
    if i == &item {
      count = count + 1
    }
  }

  count 
}

///traits 
#[derive(Debug)]
struct Mercedes{
  model: String
}

#[derive(Debug)]
struct Toyota{
  model: String
}

pub trait DisplayCar {
  fn start_engine(&self) -> String {
    format!("starting car engine")
  }

  fn dashboard(&self) -> String; 
}


impl DisplayCar for Mercedes {

  fn dashboard(&self) -> String {
   format!("welcome to {} model", &self.model) 
}
}

impl DisplayCar for Toyota {
    fn dashboard(&self) -> String {
   format!("This is toyota  {} model", &self.model) 
}
}


fn car_display(car: &impl DisplayCar) -> String {
  car.dashboard()
}





fn longest_list<'a>(list1: &'a Vec<i32>, list2: &'a Vec<i32>) -> &'a Vec<i32> {
  if list1.len() > list2.len() {
    list1 
  } else {
    list2 
  }
}