
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
struct Person{
    name : String,
    age : u32
}
impl Person {
    pub fn new(name: String,age:u32)->Self {
        Person{
            name,
            age
        }
        
    }    
}

fn main() {
    let mut people=vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort();
    assert_eq!(
        people,vec![Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
        Person::new("Zoe".to_string(), 25)]
    );
    people.sort_by(|a,b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
        Person::new("Al".to_string(), 60),
        Person::new("Zoe".to_string(), 25),
        Person::new("John".to_string(), 1)]
    )

}
