// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bit_bit() -> Query {
    t.select(t.v_bit.le(t.v_bit)).compile().into()
}

pub fn bool_bool() -> Query {
    t.select(t.v_bool.le(t.v_bool)).compile().into()
}

pub fn box_box() -> Query {
    t.select(t.v_box.le(t.v_box)).compile().into()
}

pub fn bpchar_bpchar() -> Query {
    t.select(t.v_bpchar.le(t.v_bpchar)).compile().into()
}

pub fn bytea_bytea() -> Query {
    t.select(t.v_bytea.le(t.v_bytea)).compile().into()
}

pub fn char_char() -> Query {
    t.select(t.v_char.le(t.v_char)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.le(t.v_circle)).compile().into()
}

pub fn date_date() -> Query {
    t.select(t.v_date.le(t.v_date)).compile().into()
}

pub fn date_timestamp() -> Query {
    t.select(t.v_date.le(t.v_timestamp)).compile().into()
}

pub fn date_timestamptz() -> Query {
    t.select(t.v_date.le(t.v_timestamptz)).compile().into()
}

pub fn float4_float4() -> Query {
    t.select(t.v_float4.le(t.v_float4)).compile().into()
}

pub fn float4_float8() -> Query {
    t.select(t.v_float4.le(t.v_float8)).compile().into()
}

pub fn float8_float4() -> Query {
    t.select(t.v_float8.le(t.v_float4)).compile().into()
}

pub fn float8_float8() -> Query {
    t.select(t.v_float8.le(t.v_float8)).compile().into()
}

pub fn inet_inet() -> Query {
    t.select(t.v_inet.le(t.v_inet)).compile().into()
}

pub fn int2_int2() -> Query {
    t.select(t.v_int2.le(t.v_int2)).compile().into()
}

pub fn int2_int4() -> Query {
    t.select(t.v_int2.le(t.v_int4)).compile().into()
}

pub fn int2_int8() -> Query {
    t.select(t.v_int2.le(t.v_int8)).compile().into()
}

pub fn int4_array_int4_array() -> Query {
    t.select(t.v_int4_array.le(t.v_int4_array)).compile().into()
}

pub fn int4_int2() -> Query {
    t.select(t.v_int4.le(t.v_int2)).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4.le(t.v_int4)).compile().into()
}

pub fn int4_int8() -> Query {
    t.select(t.v_int4.le(t.v_int8)).compile().into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange.le(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range.le(t.v_int4range)).compile().into()
}

pub fn int8_int2() -> Query {
    t.select(t.v_int8.le(t.v_int2)).compile().into()
}

pub fn int8_int4() -> Query {
    t.select(t.v_int8.le(t.v_int4)).compile().into()
}

pub fn int8_int8() -> Query {
    t.select(t.v_int8.le(t.v_int8)).compile().into()
}

pub fn interval_interval() -> Query {
    t.select(t.v_interval.le(t.v_interval)).compile().into()
}

pub fn jsonb_jsonb() -> Query {
    t.select(t.v_jsonb.le(t.v_jsonb)).compile().into()
}

pub fn lseg_lseg() -> Query {
    t.select(t.v_lseg.le(t.v_lseg)).compile().into()
}

pub fn macaddr8_macaddr8() -> Query {
    t.select(t.v_macaddr8.le(t.v_macaddr8)).compile().into()
}

pub fn macaddr_macaddr() -> Query {
    t.select(t.v_macaddr.le(t.v_macaddr)).compile().into()
}

pub fn money_money() -> Query {
    t.select(t.v_money.le(t.v_money)).compile().into()
}

pub fn mood_mood() -> Query {
    t.select(t.v_mood.le(t.v_mood)).compile().into()
}

pub fn numeric_numeric() -> Query {
    t.select(t.v_numeric.le(t.v_numeric)).compile().into()
}

pub fn path_path() -> Query {
    t.select(t.v_path.le(t.v_path)).compile().into()
}

pub fn price_tag_price_tag() -> Query {
    t.select(t.v_price_tag.le(t.v_price_tag)).compile().into()
}

pub fn text_array_text_array() -> Query {
    t.select(t.v_text_array.le(t.v_text_array)).compile().into()
}

pub fn text_text() -> Query {
    t.select(t.v_text.le(t.v_text)).compile().into()
}

pub fn time_time() -> Query {
    t.select(t.v_time.le(t.v_time)).compile().into()
}

pub fn timestamp_date() -> Query {
    t.select(t.v_timestamp.le(t.v_date)).compile().into()
}

pub fn timestamp_timestamp() -> Query {
    t.select(t.v_timestamp.le(t.v_timestamp)).compile().into()
}

pub fn timestamp_timestamptz() -> Query {
    t.select(t.v_timestamp.le(t.v_timestamptz)).compile().into()
}

pub fn timestamptz_date() -> Query {
    t.select(t.v_timestamptz.le(t.v_date)).compile().into()
}

pub fn timestamptz_timestamp() -> Query {
    t.select(t.v_timestamptz.le(t.v_timestamp)).compile().into()
}

pub fn timestamptz_timestamptz() -> Query {
    t.select(t.v_timestamptz.le(t.v_timestamptz))
        .compile()
        .into()
}

pub fn timetz_timetz() -> Query {
    t.select(t.v_timetz.le(t.v_timetz)).compile().into()
}

pub fn tsquery_tsquery() -> Query {
    t.select(t.v_tsquery.le(t.v_tsquery)).compile().into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange.le(t.v_tsrange)).compile().into()
}

pub fn tsvector_tsvector() -> Query {
    t.select(t.v_tsvector.le(t.v_tsvector)).compile().into()
}

pub fn uuid_uuid() -> Query {
    t.select(t.v_uuid.le(t.v_uuid)).compile().into()
}

pub fn varbit_varbit() -> Query {
    t.select(t.v_varbit.le(t.v_varbit)).compile().into()
}
