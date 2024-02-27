use rune::runtime::Object;

#[rune::function]
pub fn fetch(url: String, init: Object) {
    println!("{}", url);
    println!("{:?}", init);
}
