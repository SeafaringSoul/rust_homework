use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("Hello, world!");
    // add friends lists
    let alex = User::new("Alex".to_string());
    let bobo = User::new("Bobo".to_string());
    let pop=User::new("Pop".to_string());

    // 建立关系
    User::add_friend(&alex, &bobo);
    User::add_friend(&alex, &pop);

    // 展示
    alex.show_friends();
    bobo.show_friends();
    pop.show_friends();

}

// 用户结构
struct User{
    name:String,
    friends: RefCell<Vec<Weak<User>>>
}

impl User {
    //创建新用户
    fn new(name:String)->Rc<Self>{
        Rc::new(User{
            name:name.to_string(),
            friends: RefCell::new(vec![])
        })
    }

    // 添加朋友关系
    fn add_friend(user1: &Rc<User>, user2: &Rc<User>){
        // user2 add to user1 friends list
        user1.friends.borrow_mut().push(Rc::downgrade(user2));
        // user1 add to user2 friends list
        user2.friends.borrow_mut().push(Rc::downgrade(user1));
    }

    //显示用户列表
    fn show_friends(&self){
        println!("{}的朋友：", self.name);
        for weak_friend in self.friends.borrow().iter(){
            if let Some(friend) = weak_friend.upgrade(){
                println!(" - {}", friend.name);
            }
        }
    }
}
