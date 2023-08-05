use super::token::Token;

macro_rules! ast {
    ( $name:ident: $( $data_type:ident $field:ident ),* ) => {
        pub struct $name {
            $(
                /*
                 * Not sure if having someone pass in a proper type is the correct
                 * solution or if we should do something clever to wrap
                 * unknown type sizes in a box or whatever
                 */
                // pub $field: ast!(@ast_field $name, $data_type),
                pub $field: $data_type,
            )*
        }

        impl $name {
            pub fn new($( $field: $data_type ),*) -> $name {
                $name {
                    $( $field: $field ),*
                }
            }
        }
    };
    (@ast_field $name: ident, $data_type: ident ) => {
        Box<dyn $data_type>
    };
}
pub type Expr = Box<dyn Expression>;

pub trait Expression {}

pub enum Object {
    Number(f64),
    String(String),
}

ast!(Binary: Expr left, Token operator, Expr right);
ast!(Grouping: Expr expression);
ast!(Literal: Object value);
ast!(Unary: Token operator, Expr right);
