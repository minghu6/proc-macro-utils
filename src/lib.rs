use proc_macro2::{Ident, Span};



////////////////////////////////////////////////////////////////////////////////
//// Macro

#[macro_export]
macro_rules! ident {
    ($ctl:literal $(,)* $($arg:expr),*) => {
        {
            let s = format!($ctl, $($arg),*);
            Ident::new(
                &s,
                Span::call_site()
            )
        }
    };
}



////////////////////////////////////////////////////////////////////////////////
//// Functions

pub fn lbrace() -> Ident {
    Ident::new(
        "{",
        Span::call_site()
    )
}


pub fn rbrace() -> Ident {
    Ident::new(
        "}",
        Span::call_site()
    )
}


pub fn sign_less() -> Ident {
    Ident::new(
        "<",
        Span::call_site()
    )
}


pub fn sign_gt() -> Ident {
    Ident::new(
        ">",
        Span::call_site()
    )
}




