use std::collections::HashMap;

trait Words {
    fn get_words(&self) -> HashMap<String, crate::WordFn>;
}

trait IOWords: Words { }
trait MathWords: Words { }
trait LogicWords: Words { }
trait StackWords: Words { }
trait OtherWords: Words { }

trait StandardWords<T>: where T: IOWords + MathWords + LogicWords + StackWords + OtherWords { }