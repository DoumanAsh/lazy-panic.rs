//! Panic formatters

#[cfg(feature = "backtrace-on")]
extern crate backtrace;

use std::panic;
use std::io;

///Describes how to write panic's message prefix.
///
///Generally should be simple prefix that will go as `{Prefix} {Location}...`
pub trait Prefix {
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()>;
}

///Describes how to write panic's location
pub trait Location {
    fn write_in<W: io::Write>(writer: &mut W, info: &panic::PanicInfo) -> io::Result<()>;
}

///Describes how to write panic's payload
pub trait Payload {
    fn write_in<W: io::Write>(writer: &mut W, info: &panic::PanicInfo) -> io::Result<()>;
}

///Describes how to write panic's message suffix.
///
///Generally should be simple suffix that will go as `...{Payload} {Suffix}`
pub trait Suffix {
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()>;
}

///Describes how to write panic's backtrace
pub trait Backtrace {
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()>;
}

///Noop configuration.
///
///Literally does nothing, use it when you want to omit the part
pub struct Empty;

impl Prefix for Empty {
    #[inline]
    fn write_in<W: io::Write>(_: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl Location for Empty {
    #[inline]
    fn write_in<W: io::Write>(_: &mut W, _: &panic::PanicInfo) -> io::Result<()> {
        Ok(())
    }
}

impl Payload for Empty {
    #[inline]
    fn write_in<W: io::Write>(_: &mut W, _: &panic::PanicInfo) -> io::Result<()> {
        Ok(())
    }
}

impl Suffix for Empty {
    #[inline]
    fn write_in<W: io::Write>(_: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl Backtrace for Empty {
    #[inline]
    fn write_in<W: io::Write>(_: &mut W) -> io::Result<()> {
        Ok(())
    }
}

///Simple configuration that should be generic.
///
///For prefix it is constant string `Panic:`
///
///For payload it is writer that expects `String` and `&str`
///
///For location it is writer that formats it as `{file}:{line}`
///
///For suffix it is `\n`
///
///For backtrace it is noop
pub struct Simple;

impl Prefix for Simple {
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()> {
        writer.write_all("Panic:".as_bytes())
    }
}

impl Location for Simple {
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W, info: &panic::PanicInfo) -> io::Result<()> {
        if let Some(location) = info.location() {
            write!(writer, "{}:{}", location.file(), location.line())
        } else {
            write!(writer, "unknown:0")
        }
    }
}

impl Payload for Simple {
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W, info: &panic::PanicInfo) -> io::Result<()> {
        write_payload!(writer, info.payload(), types: [String, &str])
    }
}

impl Suffix for Simple {
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()> {
        write!(writer, "\n")
    }
}

impl Backtrace for Simple {
    #[inline]
    fn write_in<W: io::Write>(_: &mut W) -> io::Result<()> {
        Ok(())
    }
}

///Panic formatter
///
///Default print method writes each component in following order:
///1. Backtrace
///2. Prefix
///3. Location
///4. Payload
///5. Suffix
pub trait PanicFormat {
    type Writer: io::Write;
    type Backtrace: Backtrace;
    type Prefix: Prefix;
    type Location: Location;
    type Payload: Payload;
    type Suffix: Suffix;

    fn writer() -> Self::Writer;

    fn print(info: &panic::PanicInfo) {
        let mut writer = Self::writer();

        let _ = Self::Backtrace::write_in(&mut writer);
        let _ = Self::Prefix::write_in(&mut writer);
        let _ = Self::Location::write_in(&mut writer, info);
        let _ = Self::Payload::write_in(&mut writer, info);
        let _ = Self::Suffix::write_in(&mut writer);
    }
}

impl PanicFormat for Simple {
    type Writer = io::BufWriter<io::Stderr>;
    type Backtrace = Self;
    type Prefix = Self;
    type Location = Self;
    type Payload = Self;
    type Suffix = Self;

    fn writer() -> Self::Writer {
        let stderr = io::stderr();
        io::BufWriter::new(stderr)
    }
}

impl PanicFormat for Empty {
    type Writer = io::Stderr;
    type Backtrace = Self;
    type Prefix = Self;
    type Location = Self;
    type Payload = Self;
    type Suffix = Self;

    fn writer() -> Self::Writer {
        io::stderr()
    }
}


///Provides simple output with backtrace
///
///Note that if `backtrace-on` is disabled
///then `Backtrace` is noop
pub struct Debug;

impl Backtrace for Debug {
    #[cfg(not(feature = "backtrace-on"))]
    #[inline]
    fn write_in<W: io::Write>(_: &mut W) -> io::Result<()> {
        Ok(())
    }

    #[cfg(feature = "backtrace-on")]
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()> {
        let backtrace = self::backtrace::Backtrace::new();
        write!(writer, "{:?}\n", backtrace)
    }
}

impl PanicFormat for Debug {
    type Writer = io::BufWriter<io::Stderr>;
    type Prefix = Simple;
    type Location = Simple;
    type Payload = Simple;
    type Suffix = Simple;
    type Backtrace = Self;

    fn writer() -> Self::Writer {
        let stderr = io::stderr();
        io::BufWriter::new(stderr)
    }

}

#[cfg(test)]
mod tests {
    use super::{Simple, Empty, PanicFormat, Debug};

    #[test]
    #[should_panic]
    fn should_simple_panic() {
        set_panic_message!(Simple);
        panic!("lolka");
    }

    #[test]
    #[should_panic]
    fn should_empty_panic() {
        set_panic_message!(Empty);
        panic!("lolka");
    }

    #[test]
    #[should_panic]
    fn should_debug_panic() {
        set_panic_message!(Debug);
        panic!("lolka");
    }

}
