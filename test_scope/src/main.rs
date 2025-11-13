fn main() {
    let mut v: Vec<Option<i8>> = vec![None];
    v.push(Some(1));
    print!("{}", v.pop())

}
