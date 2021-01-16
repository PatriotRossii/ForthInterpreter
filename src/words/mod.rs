use crate::Result;
use std::collections::HashMap;

pub trait IOWords {
    fn print_top(&mut self) -> Result<()>;
    fn emit(&mut self) -> Result<()>;
    fn cr(&mut self) -> Result<()>;

    fn get_words() -> Vec<(String, crate::WordFn)> {
        vec![
            (".".into(), IOWords::print_top as crate::WordFn),
            ("emit".into(), IOWords::emit as crate::WordFn),
            ("cr".into(), IOWords::cr as crate::WordFn),
        ]
    }
}

pub trait MathWords {
    fn add(&mut self) -> Result<()>;
    fn sub(&mut self) -> Result<()>;
    fn mul(&mut self) -> Result<()>;
    fn div(&mut self) -> Result<()>;
    fn r#mod(&mut self) -> Result<()>;

    fn negate(&mut self) -> Result<()>;
    fn abs(&mut self) -> Result<()>;
    fn max(&mut self) -> Result<()>;
    fn min(&mut self) -> Result<()>;

    fn get_words() -> Vec<(String, crate::WordFn)> {
        vec![
            ("+".into(), MathWords::add as crate::WordFn),
            ("-".into(), MathWords::sub as crate::WordFn),
            ("*".into(), MathWords::mul as crate::WordFn),
            ("/".into(), MathWords::div as crate::WordFn),
            ("mod".into(), MathWords::r#mod as crate::WordFn),
            
            ("negate".into(), MathWords::negate as crate::WordFn),
            ("abs".into(), MathWords::abs as crate::WordFn),
            ("max".into(), MathWords::max as crate::WordFn),
            ("min".into(), MathWords::min as crate::WordFn),
        ]
    }
}

pub trait LogicWords {
    fn equal(&mut self) -> Result<()>;
    fn greater(&mut self) -> Result<()>;
    fn less(&mut self) -> Result<()>;

    fn not(&mut self) -> Result<()>;
    fn and(&mut self) -> Result<()>;
    fn or(&mut self) -> Result<()>;

    fn get_words() -> Vec<(String, crate::WordFn)> {
        vec![
            ("=".into(), LogicWords::equal as crate::WordFn),
            (">".into(), LogicWords::greater as crate::WordFn),
            ("<".into(), LogicWords::less as crate::WordFn),

            ("!".into(), LogicWords::not as crate::WordFn),
            ("and".into(), LogicWords::and as crate::WordFn),
            ("or".into(), LogicWords::or as crate::WordFn),
        ]
    }
}

pub trait StackWords { 
    fn dup(&mut self) -> Result<()>;
    fn drop(&mut self) -> Result<()>;
    fn swap(&mut self) -> Result<()>;
    fn over(&mut self) -> Result<()>;
    fn rot(&mut self) -> Result<()>;

    fn fetch_variable(&mut self) -> Result<()>;

    fn get_words() -> Vec<(String, crate::WordFn)> {
        vec![
            ("dup".into(), StackWords::dup as crate::WordFn),
            ("drop".into(), StackWords::drop as crate::WordFn),
            ("swap".into(), StackWords::swap as crate::WordFn),

            ("over".into(), StackWords::over as crate::WordFn),
            ("rot".into(), StackWords::rot as crate::WordFn),
            ("@".into(), StackWords::fetch_variable as crate::WordFn),
        ]
    }
}

pub trait OtherWords {
    fn store_variable(&mut self) -> Result<()>;

    fn cells(&mut self) -> Result<()>;
    fn allot(&mut self) -> Result<()>;

    fn get_words() -> Vec<(String, crate::WordFn)> {
        vec![
            ("!".into(), OtherWords::store_variable as crate::WordFn),
            
            ("cells".into(), OtherWords::cells as crate::WordFn),
            ("allot".into(), OtherWords::allot as crate::WordFn),
        ]
    }
}


pub trait StandardWords where Self: IOWords + MathWords + LogicWords + StackWords + OtherWords {
    fn get_words() -> HashMap<String, crate::WordFn> {
        <Self as IOWords>::get_words().iter().chain(
            <Self as MathWords>::get_words().iter()
        ).chain(
          <Self as LogicWords>::get_words().iter()  
        ).chain(
            <Self as StackWords>::get_words().iter()
        ).chain(
            <Self as OtherWords>::get_words().iter()
        ).cloned().collect()
    }
}