use super::rnd;

pub type Generator = fn() -> String;

pub const ALL: [(&str, Generator); 47] = [
    ("bool", bool),
    ("int2", int2),
    ("int4", int4),
    ("int8", int8),
    ("numeric", numeric),
    ("float4", float4),
    ("float8", float8),
    ("money", money),
    ("text", text),
    ("varchar", varchar),
    ("bpchar", bpchar),
    ("char", char),
    ("date", date),
    ("time", time),
    ("timetz", timetz),
    ("timestamp", timestamp),
    ("timestamptz", timestamptz),
    ("interval", interval),
    ("bytea", bytea),
    ("uuid", uuid),
    ("json", json),
    ("jsonb", jsonb),
    ("jsonpath", jsonpath),
    ("inet", inet),
    ("cidr", cidr),
    ("macaddr", macaddr),
    ("macaddr8", macaddr8),
    ("bit", bit),
    ("varbit", varbit),
    ("point", point),
    ("line", line),
    ("lseg", lseg),
    ("box", r#box),
    ("circle", circle),
    ("path", path),
    ("polygon", polygon),
    ("tsvector", tsvector),
    ("tsquery", tsquery),
    ("xml", xml),
    ("int4_array", int4_array),
    ("text_array", text_array),
    ("int4range", int4range),
    ("tsrange", tsrange),
    ("int4multirange", int4multirange),
    ("mood", mood),
    ("positive_int", positive_int),
    ("price_tag", price_tag),
];

fn n(limit: u64) -> u64 {
    rnd::u64() % limit
}

fn signed(limit: u64) -> i64 {
    (n(2 * limit) as i128 - limit as i128) as i64
}

fn hex(bytes: usize) -> String {
    (0..bytes).map(|_| format!("{:02x}", n(256))).collect()
}

fn coord() -> String {
    format!("({},{})", signed(1000), signed(1000))
}

pub fn bool() -> String {
    ["true", "false"][n(2) as usize].to_string()
}

pub fn int2() -> String {
    signed(i16::MAX as u64).to_string()
}

pub fn int4() -> String {
    signed(i32::MAX as u64).to_string()
}

pub fn int8() -> String {
    signed(i64::MAX as u64).to_string()
}

pub fn numeric() -> String {
    format!("{}.{:04}", signed(1_000_000_000), n(10_000))
}

pub fn float4() -> String {
    format!("{}.25", signed(10_000))
}

pub fn float8() -> String {
    format!("{}.125", signed(1_000_000_000))
}

pub fn money() -> String {
    format!("{}.{:02}", n(1_000_000), n(100))
}

pub fn text() -> String {
    rnd::text()
}

pub fn varchar() -> String {
    rnd::text()
}

pub fn bpchar() -> String {
    rnd::text()
}

pub fn char() -> String {
    char::from(b'a' + n(26) as u8).to_string()
}

pub fn date() -> String {
    format!("{}-{:02}-{:02}", 1970 + n(100), 1 + n(12), 1 + n(28))
}

pub fn time() -> String {
    format!("{:02}:{:02}:{:02}", n(24), n(60), n(60))
}

pub fn timetz() -> String {
    format!("{}+{:02}", time(), n(13))
}

pub fn timestamp() -> String {
    format!("{} {}", date(), time())
}

pub fn timestamptz() -> String {
    format!("{} {}+00", date(), time())
}

pub fn interval() -> String {
    format!("{} days {} hours {} minutes", n(1000), n(24), n(60))
}

pub fn bytea() -> String {
    format!("\\x{}", hex(16))
}

pub fn uuid() -> String {
    format!("{}-{}-{}-{}-{}", hex(4), hex(2), hex(2), hex(2), hex(6))
}

pub fn json() -> String {
    format!(
        "{{\"k\": {}, \"s\": \"{}\"}}",
        signed(1_000_000),
        rnd::text()
    )
}

pub fn jsonb() -> String {
    json()
}

pub fn jsonpath() -> String {
    format!("$.items[{}].name", n(100))
}

pub fn inet() -> String {
    format!("10.{}.{}.{}", n(256), n(256), n(256))
}

pub fn cidr() -> String {
    format!("10.{}.{}.0/24", n(256), n(256))
}

pub fn macaddr() -> String {
    (0..6)
        .map(|_| format!("{:02x}", n(256)))
        .collect::<Vec<_>>()
        .join(":")
}

pub fn macaddr8() -> String {
    (0..8)
        .map(|_| format!("{:02x}", n(256)))
        .collect::<Vec<_>>()
        .join(":")
}

pub fn bit() -> String {
    format!("{:08b}", n(256))
}

pub fn varbit() -> String {
    format!("{:b}", 1 + n(u16::MAX as u64))
}

pub fn point() -> String {
    coord()
}

pub fn line() -> String {
    format!("{{{},{},{}}}", 1 + n(100), 1 + n(100), signed(1000))
}

pub fn lseg() -> String {
    format!("[{},{}]", coord(), coord())
}

pub fn r#box() -> String {
    format!("({},{})", coord(), coord())
}

pub fn circle() -> String {
    format!("<{},{}>", coord(), 1 + n(100))
}

pub fn path() -> String {
    format!("[{},{},{}]", coord(), coord(), coord())
}

pub fn polygon() -> String {
    format!("({},{},{})", coord(), coord(), coord())
}

pub fn tsvector() -> String {
    format!("w{} w{} w{}", n(1000), n(1000), n(1000))
}

pub fn tsquery() -> String {
    format!("w{} & w{} | !w{}", n(1000), n(1000), n(1000))
}

pub fn xml() -> String {
    format!("<item id=\"{}\">{}</item>", n(1000), rnd::text())
}

pub fn int4_array() -> String {
    format!("{{{},{},{}}}", signed(1000), signed(1000), signed(1000))
}

pub fn text_array() -> String {
    format!("{{{},{}}}", rnd::text(), rnd::text())
}

pub fn int4range() -> String {
    let low = signed(1000);
    format!("[{low},{})", low + 1 + n(100) as i64)
}

pub fn tsrange() -> String {
    format!(
        "[{}-01-01 00:00:00,{}-01-01 00:00:00)",
        1970 + n(50),
        2020 + n(50)
    )
}

pub fn int4multirange() -> String {
    let low = signed(1000);
    format!("{{[{low},{}),[{},{})}}", low + 2, low + 10, low + 20)
}

pub fn mood() -> String {
    ["sad", "ok", "happy"][n(3) as usize].to_string()
}

pub fn positive_int() -> String {
    (1 + n(1000)).to_string()
}

pub fn price_tag() -> String {
    format!(
        "({}.{:02},{})",
        n(1000),
        n(100),
        ["USD", "EUR", "CHF"][n(3) as usize]
    )
}
