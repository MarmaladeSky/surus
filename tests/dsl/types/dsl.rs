use crate::support::{Param, Query};

pub fn bool(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int2(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int8(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn numeric(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn float4(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn float8(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn money(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn text(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn varchar(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn bpchar(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn char(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn date(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn time(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn timetz(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn timestamp(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn timestamptz(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn interval(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn bytea(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn uuid(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn json(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn jsonb(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn jsonpath(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn inet(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn cidr(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn macaddr(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn macaddr8(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn bit(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn varbit(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn point(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn line(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn lseg(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn r#box(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn circle(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn path(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn polygon(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsvector(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsquery(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn xml(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4_array(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn text_array(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4range(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn tsrange(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn int4multirange(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn mood(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn positive_int(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}

pub fn price_tag(_value: Param) -> Query {
    Query::plain("SELECT NULL WHERE false")
}
