/*pub struct Pallet<T: PalletType<T>> {
    t: T,
    content: Box<T>,
}
impl<T> Pallet<T: PalletType<T>> {}

trait PalletType<T> {}

#[derive(Debug)]
struct Empty {}
impl PalletType<()> for Empty {}

#[derive(Debug)]
struct Bool {
    contents: bool,
}
impl PalletType<bool> for Bool {}
*/
pub enum Pallet {
    Empty,
    Bool(bool),
    Char(char),
    String(String),
    Int(i32),
    Float(f32),
}
