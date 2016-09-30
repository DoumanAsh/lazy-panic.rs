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
///format_payload!(payload, String, &'static str)
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
