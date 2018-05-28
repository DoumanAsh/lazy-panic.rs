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
///* ```payload_types``` - Types of used messages in panics.
///* ```suffix``` - Part of trace appended to panic message.
///* ```prefix``` - Part of trace pre-pended to panic message.
///
///# Format
///
///`{prefix}{message}{suffix}`
///where:
///
///- message is `{file}:{line} -`
#[macro_export]
macro_rules! set_panic_message {
    ($config:ty) => {{
        use std::panic;
        type Printer = $config;
        panic::set_hook(Box::new(move |info| {
            Printer::print(&info);
        }))
    }}
}

pub mod formatter;
