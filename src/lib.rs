//一个用来记录信息的trait
pub trait Messenger {
    fn send(&self,msg: &str);
}


//检查磁盘容量的一个结构体，T是任何实现了Messenger的类型，用于send消息
#[allow(unused)]
pub struct LimitTracker<'a, T:Messenger> {
    messenger: &'a T, //这是一个T类型的引用，就像一个指针一样，策略模式？
    value: usize, //当前值
    max: usize,   //最大值
}

impl<'a,T> LimitTracker<'a,T>
    where T: Messenger
{
    //new 方法，自己看一下即可
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    //这个方法能够设置val，并且在超过对应限度的时候使用messenger的send方法，发送一条信息。
    //由于这个方法是默认返回类型(),如果想对这个功能进行测试，需要一些手法,下面再说
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quata!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, borrow::Borrow};
    //简单的一个结构体，仅仅对Vec进行了一次包装
    struct MockMessenger {
        sent_messenger: RefCell<Vec<String>>,
    }

    //关联函数new
    impl MockMessenger {
        fn new() -> Self {
            MockMessenger { sent_messenger: RefCell::new(vec![]) }
        }
    }

    //实现Messenger trait
    impl Messenger for MockMessenger {
        fn send(&self,msg: &str) {
            //当接受到了一个消息，将其放在sent_messenger(type: Vec<String>)中
            self.sent_messenger.borrow_mut().push(String::from(msg)); //这里会出现问题
        }
    }

    #[test]
    //用于断言测试超过%75，是否发送一个消息
    fn it_sends_an_over_75_percent_waring_message() {
        let mock_messenger = MockMessenger::new();
        //这里将messanger设置为上面的MockMessenger的一个变量，这样在每次send消息
        //Vec中就会push一次，我们就可以使用len方法来断言是否成功发送消息
        //这种手法就是替身测试
        let mut limit_tracker = LimitTracker::new(&mock_messenger,100);

        limit_tracker.set_value(100);
        assert_eq!(mock_messenger.sent_messenger.borrow().len(), 1);
    }
}
