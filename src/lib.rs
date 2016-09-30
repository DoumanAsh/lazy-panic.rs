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
///```String```. For not specified types it is `Any`
///
///# Usage
///
///```
///#[macro_use]
///extern crate lazy_panic;
///
///fn main() {
///    std::panic::set_hook(Box::new(|info| {
///        let payload = info.payload();
///
///        let msg = format!("{}{}",
///                          info.location()
///                              .map(|loc| format!("{}:{} - ", loc.file(), loc.line()))
///                              .unwrap_or("".to_string()),
///                          format_payload!(payload, String, &'static str));
///
///        println!("Fatal error: {}", msg);
///     }));
///}
///```
#[macro_export]
macro_rules! format_payload {
    ($payload:expr, $($p_type:ty),+) => {{
        $(
            if let Some(result) = $payload.downcast_ref::<$p_type>() {
                format!("{}", result)
            }
         )else+
         else {
             format!("{:?}", $payload)
         }
    }};
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
    (payload_types=>$($p_type:ty),+) => {{
        set_panic_hook!(prefix=>"", suffix=>"", payload_types=>$($p_type),+)
    }};
    (suffix=>$suffix:expr, payload_types=>$($p_type:ty),+) => {{
        set_panic_hook!(prefix=>"", suffix=>$suffix, payload_types=>$($p_type),+)
    }};
    (prefix=>$prefix:expr, payload_types=>$($p_type:ty),+) => {{
        set_panic_hook!(prefix=>$prefix, suffix=>"", payload_types=>$($p_type),+)
    }};
    (suffix=>$suffix:expr, prefix=>$prefix:expr, payload_types=>$($p_type:ty),+) => {{
        set_panic_hook!(prefix=>$prefix, suffix=>$suffix, payload_types=>$($p_type),+)
    }};
    (prefix=>$prefix:expr, suffix=>$suffix:expr, payload_types=>$($p_type:ty),+) => {{
        std::panic::set_hook(Box::new(|info| {
            let payload = info.payload();

            let msg = format!("{}{}",
                              info.location()
                                  .map(|loc| format!("{}:{} - ", loc.file(), loc.line()))
                                  .unwrap_or("".to_string()),
                              format_payload!(payload, $($p_type),+));

            println!("{}{}{}", $prefix, msg, $suffix);
        }))
    }}
}
