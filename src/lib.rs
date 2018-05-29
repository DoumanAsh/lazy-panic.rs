//! Provides lazy utilities to lazily set custom panic hook.

///Formats ```PanicInfo``` payload into ```String```
///
///# Arguments
///
///* ```payload``` - ```PanicInfo``` payload message.
///* ```p_type``` - Multiple number of types which payload can be. If not among these types then it
///is formatted as ```{:?}```
///
///# Return
///
///Result of `write!` macro
#[macro_export]
macro_rules! write_payload {
    ($writer:expr, $payload:expr, types: [$($p_type:ty),+]) => {{
        $(
            if let Some(result) = $payload.downcast_ref::<$p_type>() {
                write!($writer, "{}", result)
            }
         )else+
         else {
             write!($writer, "{:?}", $payload)
         }
    }}
}

///Sets custom printer for panic.
///
///# Arguments
///
///* ```Config``` - panic formatter that implements [PanicFormat](formatter/trait.PanicFormat.html)
#[macro_export]
macro_rules! set_panic_message {
    ($config:ty) => {{
        use std::panic;
        use $crate::formatter::PanicFormat;
        type Printer = $config;
        panic::set_hook(Box::new(move |info| {
            Printer::print(&info);
        }))
    }}
}

pub mod formatter;
