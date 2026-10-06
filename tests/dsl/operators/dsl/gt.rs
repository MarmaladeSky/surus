// Each operator is applied to columns of `type_samples`, often the same one twice.
#![allow(clippy::eq_op)]

use crate::support::Query;
use schema::type_samples as t;
use surus::*;

pub fn bit_bit() -> Query {
    t.select(t.v_bit.gt(t.v_bit)).compile().into()
}

pub fn bool_bool() -> Query {
    t.select(t.v_bool.gt(t.v_bool)).compile().into()
}

pub fn box_box() -> Query {
    t.select(t.v_box.gt(t.v_box)).compile().into()
}

pub fn bpchar_bpchar() -> Query {
    t.select(t.v_bpchar.gt(t.v_bpchar)).compile().into()
}

pub fn bytea_bytea() -> Query {
    t.select(t.v_bytea.gt(t.v_bytea)).compile().into()
}

pub fn char_char() -> Query {
    t.select(t.v_char.gt(t.v_char)).compile().into()
}

pub fn circle_circle() -> Query {
    t.select(t.v_circle.gt(t.v_circle)).compile().into()
}

pub fn date_date() -> Query {
    t.select(t.v_date.gt(t.v_date)).compile().into()
}

pub fn date_timestamp() -> Query {
    t.select(t.v_date.gt(t.v_timestamp)).compile().into()
}

pub fn date_timestamptz() -> Query {
    t.select(t.v_date.gt(t.v_timestamptz)).compile().into()
}

pub fn float4_float4() -> Query {
    t.select(t.v_float4.gt(t.v_float4)).compile().into()
}

pub fn float4_float8() -> Query {
    t.select(t.v_float4.gt(t.v_float8)).compile().into()
}

pub fn float8_float4() -> Query {
    t.select(t.v_float8.gt(t.v_float4)).compile().into()
}

pub fn float8_float8() -> Query {
    t.select(t.v_float8.gt(t.v_float8)).compile().into()
}

pub fn inet_inet() -> Query {
    t.select(t.v_inet.gt(t.v_inet)).compile().into()
}

pub fn int2_int2() -> Query {
    t.select(t.v_int2.gt(t.v_int2)).compile().into()
}

pub fn int2_int4() -> Query {
    t.select(t.v_int2.gt(t.v_int4)).compile().into()
}

pub fn int2_int8() -> Query {
    t.select(t.v_int2.gt(t.v_int8)).compile().into()
}

pub fn int4_array_int4_array() -> Query {
    t.select(t.v_int4_array.gt(t.v_int4_array)).compile().into()
}

pub fn int4_int2() -> Query {
    t.select(t.v_int4.gt(t.v_int2)).compile().into()
}

pub fn int4_int4() -> Query {
    t.select(t.v_int4.gt(t.v_int4)).compile().into()
}

pub fn int4_int8() -> Query {
    t.select(t.v_int4.gt(t.v_int8)).compile().into()
}

pub fn int4multirange_int4multirange() -> Query {
    t.select(t.v_int4multirange.gt(t.v_int4multirange))
        .compile()
        .into()
}

pub fn int4range_int4range() -> Query {
    t.select(t.v_int4range.gt(t.v_int4range)).compile().into()
}

pub fn int8_int2() -> Query {
    t.select(t.v_int8.gt(t.v_int2)).compile().into()
}

pub fn int8_int4() -> Query {
    t.select(t.v_int8.gt(t.v_int4)).compile().into()
}

pub fn int8_int8() -> Query {
    t.select(t.v_int8.gt(t.v_int8)).compile().into()
}

pub fn interval_interval() -> Query {
    t.select(t.v_interval.gt(t.v_interval)).compile().into()
}

pub fn jsonb_jsonb() -> Query {
    t.select(t.v_jsonb.gt(t.v_jsonb)).compile().into()
}

pub fn lseg_lseg() -> Query {
    t.select(t.v_lseg.gt(t.v_lseg)).compile().into()
}

pub fn macaddr8_macaddr8() -> Query {
    t.select(t.v_macaddr8.gt(t.v_macaddr8)).compile().into()
}

pub fn macaddr_macaddr() -> Query {
    t.select(t.v_macaddr.gt(t.v_macaddr)).compile().into()
}

pub fn money_money() -> Query {
    t.select(t.v_money.gt(t.v_money)).compile().into()
}

pub fn mood_mood() -> Query {
    t.select(t.v_mood.gt(t.v_mood)).compile().into()
}

pub fn numeric_numeric() -> Query {
    t.select(t.v_numeric.gt(t.v_numeric)).compile().into()
}

pub fn path_path() -> Query {
    t.select(t.v_path.gt(t.v_path)).compile().into()
}

pub fn price_tag_price_tag() -> Query {
    t.select(t.v_price_tag.gt(t.v_price_tag)).compile().into()
}

pub fn text_array_text_array() -> Query {
    t.select(t.v_text_array.gt(t.v_text_array)).compile().into()
}

pub fn text_text() -> Query {
    t.select(t.v_text.gt(t.v_text)).compile().into()
}

pub fn time_time() -> Query {
    t.select(t.v_time.gt(t.v_time)).compile().into()
}

pub fn timestamp_date() -> Query {
    t.select(t.v_timestamp.gt(t.v_date)).compile().into()
}

pub fn timestamp_timestamp() -> Query {
    t.select(t.v_timestamp.gt(t.v_timestamp)).compile().into()
}

pub fn timestamp_timestamptz() -> Query {
    t.select(t.v_timestamp.gt(t.v_timestamptz)).compile().into()
}

pub fn timestamptz_date() -> Query {
    t.select(t.v_timestamptz.gt(t.v_date)).compile().into()
}

pub fn timestamptz_timestamp() -> Query {
    t.select(t.v_timestamptz.gt(t.v_timestamp)).compile().into()
}

pub fn timestamptz_timestamptz() -> Query {
    t.select(t.v_timestamptz.gt(t.v_timestamptz))
        .compile()
        .into()
}

pub fn timetz_timetz() -> Query {
    t.select(t.v_timetz.gt(t.v_timetz)).compile().into()
}

pub fn tsquery_tsquery() -> Query {
    t.select(t.v_tsquery.gt(t.v_tsquery)).compile().into()
}

pub fn tsrange_tsrange() -> Query {
    t.select(t.v_tsrange.gt(t.v_tsrange)).compile().into()
}

pub fn tsvector_tsvector() -> Query {
    t.select(t.v_tsvector.gt(t.v_tsvector)).compile().into()
}

pub fn uuid_uuid() -> Query {
    t.select(t.v_uuid.gt(t.v_uuid)).compile().into()
}

pub fn varbit_varbit() -> Query {
    t.select(t.v_varbit.gt(t.v_varbit)).compile().into()
}
