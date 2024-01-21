use crate::interpreter::variable::Unit;

use crate::interpreter::scope::TowerScope;


pub enum Element<'a> {
    String(&'a str),
    Expr(Unit<'a>)
}
impl<'a> Element<'a> {
    pub fn print(&self, scopes: &TowerScope<'a>) {
        match self {
            Self::String(string) => println!("{}", string),
            Self::Expr(unit) => unit.print(scopes),
        }
    }
}

// impl<'a> fmt::Display for Element<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::String(string) => f.write_str(string),
//             Self::Expr(var) => f.write_str(&var.to_string()),
//         }
//     }
// }