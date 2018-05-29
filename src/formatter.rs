//! Formatter module for Panic related messages

#[cfg(feature = "backtrace-on")]
extern crate backtrace;

use std::panic;
use std::io;

///Describes how to write panic's message prefix.
///
///Generally should be simple prefix that will go as `{Prefix}{PanicInfo}...`
pub trait Prefix {
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()>;
}

///Describes how to write `PanicInfo`
pub trait PanicInfo {
    fn write_in<W: io::Write>(writer: &mut W, info: &panic::PanicInfo) -> io::Result<()>;
}

///Describes how to write panic's message suffix.
///
///Generally should be simple suffix that will go as `...{PanicInfo}{Suffix}`
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
///
///As [PanicFormat](trait.PanicFormat.html) it writes nothing
pub struct Empty;

impl Prefix for Empty {
    #[inline]
    fn write_in<W: io::Write>(_: &mut W) -> io::Result<()> {
        Ok(())
    }
}

impl PanicInfo for Empty {
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
///For prefix it is constant string `Panic: `
///
///For `PanicInfo` it writes `{file}:{line} - {payload}`
///
///For suffix it is `\n`
///
///For backtrace it is noop
///
///As [PanicFormat](trait.PanicFormat.html) all above together.
pub struct Simple;

impl Prefix for Simple {
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W) -> io::Result<()> {
        writer.write_all("Panic: ".as_bytes())
    }
}

impl PanicInfo for Simple {
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W, info: &panic::PanicInfo) -> io::Result<()> {
        match info.location() {
            Some(location) => write!(writer, "{}:{} - ", location.file(), location.line()),
            None  => write!(writer, "unknown:0 - ")
        }?;
        write_payload!(writer, info.payload(), types: [&str, String])
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
///3. `PanicInfo`
///5. Suffix
pub trait PanicFormat {
    type Writer: io::Write;
    type Backtrace: Backtrace;
    type Prefix: Prefix;
    type PanicInfo: PanicInfo;
    type Suffix: Suffix;

    fn writer() -> Self::Writer;

    fn print(info: &panic::PanicInfo) {
        let mut writer = Self::writer();

        let _ = Self::Backtrace::write_in(&mut writer);
        let _ = Self::Prefix::write_in(&mut writer);
        let _ = Self::PanicInfo::write_in(&mut writer, info);
        let _ = Self::Suffix::write_in(&mut writer);
    }
}

impl PanicFormat for Simple {
    type Writer = io::BufWriter<io::Stderr>;
    type Backtrace = Self;
    type Prefix = Self;
    type PanicInfo = Self;
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
    type PanicInfo = Self;
    type Suffix = Self;

    fn writer() -> Self::Writer {
        io::stderr()
    }

    fn print(_: &panic::PanicInfo) {
    }
}


///Provides simple output with backtrace
///
///Note that if `backtrace-on` is disabled
///then `Backtrace` is noop
///
///Note: Backtrace output is trimmed to only user's most recent call
///So actual call stack may be longer
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
        use std::mem;

        //First 3 frames are from backtrace.
        //In middle 3 are from lazy_panic
        //Last 2 are from Rust runtime
        const TRASH_FRAMES_NUM: usize = 8;
        const HEX_WIDTH: usize = mem::size_of::<usize>() * 2 + 2;

        let backtrace = self::backtrace::Backtrace::new();
        //By default backtrace includes last function call
        //which means the above new()
        //But we should really trim it down to user panic

        //Code is based on backtrace source
        write!(writer, "Stack backtrace:")?;
        for (idx, frame) in backtrace.frames().iter().skip(TRASH_FRAMES_NUM).enumerate() {
            let ip = frame.ip();
            write!(writer, "\n{:4}: {:2$?}", idx, ip, HEX_WIDTH)?;

            let symbols = frame.symbols();
            if symbols.len() == 0 {
                write!(writer, " - <unresolved>")?;
            }

            for (idx, symbol) in symbols.iter().enumerate() {
                if idx != 0 {
                    write!(writer, "\n      {:1$}", "", HEX_WIDTH)?;
                }

                if let Some(name) = symbol.name() {
                    write!(writer, " - {}", name)?;
                } else {
                    write!(writer, " - <unknown>")?;
                }

                if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                    write!(writer, "\n      {:3$}at {}:{}", "", file.display(), line, HEX_WIDTH)?;
                }
            }
        }

        write!(writer, "\n")
    }
}

impl PanicFormat for Debug {
    type Writer = io::BufWriter<io::Stderr>;
    type Prefix = Simple;
    type PanicInfo = Simple;
    type Suffix = Simple;
    type Backtrace = Self;

    fn writer() -> Self::Writer {
        let stderr = io::stderr();
        io::BufWriter::new(stderr)
    }
}

///Treats panic as just error
///
///Only panic's payload gets printed to stderr
pub struct JustError;

impl PanicInfo for JustError {
    #[inline]
    fn write_in<W: io::Write>(writer: &mut W, info: &panic::PanicInfo) -> io::Result<()> {
        write_payload!(writer, info.payload(), types: [&str, String])
    }
}

impl PanicFormat for JustError {
    type Writer = io::BufWriter<io::Stderr>;
    type Prefix = Empty;
    type PanicInfo = Self;
    type Suffix = Simple;
    type Backtrace = Empty;

    fn writer() -> Self::Writer {
        let stderr = io::stderr();
        io::BufWriter::new(stderr)
    }
}

#[cfg(test)]
mod tests {
    use super::{Simple, Empty, Debug, JustError};

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

    #[test]
    #[should_panic]
    fn should_just_error_panic() {
        set_panic_message!(JustError);
        panic!("lolka");
    }

}
