use std::io;

use io::BufWriter;
use io::Write;

use emojis::Emoji;

#[derive(serde::Serialize)]
pub struct UnicodeVer {
    pub major: u32,
    pub minor: u32,
}

#[derive(serde::Serialize)]
pub struct EmojiInfo<'a> {
    pub name: &'a str,
    pub group: String,
    pub short_codes: Vec<String>,
    pub skin_tone: Option<String>,
    pub unicode_version: UnicodeVer,
}

impl<'a> From<&'a Emoji> for EmojiInfo<'a> {
    fn from(e: &'a Emoji) -> Self {
        Self {
            name: e.name(),
            group: format!("{:?}", e.group()),
            short_codes: e.shortcodes().map(|s| s.into()).collect(),
            skin_tone: e.skin_tone().map(|s| format!("{s:?}")),
            unicode_version: UnicodeVer {
                major: e.unicode_version().major(),
                minor: e.unicode_version().minor(),
            },
        }
    }
}

pub fn print_all_simple<W>(mut wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    let all = emojis::iter();
    for em in all {
        writeln!(&mut wtr, "{em}")?;
    }
    wtr.flush()
}

pub fn print_all_simple_stdout() -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    print_all_simple(BufWriter::new(&mut ol))?;
    ol.flush()
}

pub fn print_all_with_formatter<W, F>(mut wtr: W, fmt: F) -> Result<(), io::Error>
where
    W: Write,
    F: Fn(&mut W, &Emoji) -> Result<(), io::Error>,
{
    let all = emojis::iter();
    for em in all {
        fmt(&mut wtr, em)?;
    }
    wtr.flush()
}

pub fn emoji2wtr<W>(mut wtr: &mut W, e: &Emoji) -> Result<(), io::Error>
where
    W: Write,
{
    let einfo: EmojiInfo = e.into();
    serde_json::to_writer(&mut wtr, &einfo).map_err(io::Error::other)?;
    writeln!(wtr)
}

pub fn print_all_as_json<W>(wtr: W) -> Result<(), io::Error>
where
    W: Write,
{
    print_all_with_formatter(wtr, emoji2wtr)
}

pub fn print_all_as_json2stdout() -> Result<(), io::Error> {
    let o = io::stdout();
    let mut ol = o.lock();
    print_all_as_json(BufWriter::new(&mut ol))?;
    ol.flush()
}
