

pub struct Guomi {
}

impl Guomi {
    pub fn sm2() {
        println!("This is guomi sm2 alg.");
    }

    pub fn sm3() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
