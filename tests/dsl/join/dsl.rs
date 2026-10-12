use crate::schema::*;
use surus::prelude::*;
use surus::types::{Bool, Int4, Int8, Nullable, Text};
use surus::{
    Query, coalesce, count_star, generate_series, query, rows_from, string_to_array, unnest, values,
};

pub fn users_with_group() -> Query<(), (Text, Text)> {
    users
        .join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.name, g.name))
        .order_by(|(u, _)| u.name)
        .into()
}

pub fn group_ancestor_30_levels() -> Query<(), (Text, Text)> {
    groups
        .join(groups, |(g0, g1)| g1.id.eq(g0.parent_id))
        .join(groups, |(.., g1, g2)| g2.id.eq(g1.parent_id))
        .join(groups, |(.., g2, g3)| g3.id.eq(g2.parent_id))
        .join(groups, |(.., g3, g4)| g4.id.eq(g3.parent_id))
        .join(groups, |(.., g4, g5)| g5.id.eq(g4.parent_id))
        .join(groups, |(.., g5, g6)| g6.id.eq(g5.parent_id))
        .join(groups, |(.., g6, g7)| g7.id.eq(g6.parent_id))
        .join(groups, |(.., g7, g8)| g8.id.eq(g7.parent_id))
        .join(groups, |(.., g8, g9)| g9.id.eq(g8.parent_id))
        .join(groups, |(.., g9, g10)| g10.id.eq(g9.parent_id))
        .join(groups, |(.., g10, g11)| g11.id.eq(g10.parent_id))
        .join(groups, |(.., g11, g12)| g12.id.eq(g11.parent_id))
        .join(groups, |(.., g12, g13)| g13.id.eq(g12.parent_id))
        .join(groups, |(.., g13, g14)| g14.id.eq(g13.parent_id))
        .join(groups, |(.., g14, g15)| g15.id.eq(g14.parent_id))
        .join(groups, |(.., g15, g16)| g16.id.eq(g15.parent_id))
        .join(groups, |(.., g16, g17)| g17.id.eq(g16.parent_id))
        .join(groups, |(.., g17, g18)| g18.id.eq(g17.parent_id))
        .join(groups, |(.., g18, g19)| g19.id.eq(g18.parent_id))
        .join(groups, |(.., g19, g20)| g20.id.eq(g19.parent_id))
        .join(groups, |(.., g20, g21)| g21.id.eq(g20.parent_id))
        .join(groups, |(.., g21, g22)| g22.id.eq(g21.parent_id))
        .join(groups, |(.., g22, g23)| g23.id.eq(g22.parent_id))
        .join(groups, |(.., g23, g24)| g24.id.eq(g23.parent_id))
        .join(groups, |(.., g24, g25)| g25.id.eq(g24.parent_id))
        .join(groups, |(.., g25, g26)| g26.id.eq(g25.parent_id))
        .join(groups, |(.., g26, g27)| g27.id.eq(g26.parent_id))
        .join(groups, |(.., g27, g28)| g28.id.eq(g27.parent_id))
        .join(groups, |(.., g28, g29)| g29.id.eq(g28.parent_id))
        .join(groups, |(.., g29, g30)| g30.id.eq(g29.parent_id))
        .select(|(g0, .., g30)| (g0.name, g30.name))
        .into()
}

pub fn logins_30_day_pivot() -> Query<(), super::Logins> {
    users
        .join(logins_day_01, |(u, d)| d.user_id.eq(u.id))
        .join(logins_day_02, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_03, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_04, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_05, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_06, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_07, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_08, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_09, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_10, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_11, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_12, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_13, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_14, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_15, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_16, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_17, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_18, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_19, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_20, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_21, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_22, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_23, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_24, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_25, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_26, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_27, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_28, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_29, |(u, .., d)| d.user_id.eq(u.id))
        .join(logins_day_30, |(u, .., d)| d.user_id.eq(u.id))
        .select(
            |(
                u,
                d01,
                d02,
                d03,
                d04,
                d05,
                d06,
                d07,
                d08,
                d09,
                d10,
                d11,
                d12,
                d13,
                d14,
                d15,
                d16,
                d17,
                d18,
                d19,
                d20,
                d21,
                d22,
                d23,
                d24,
                d25,
                d26,
                d27,
                d28,
                d29,
                d30,
            )| {
                (
                    u.name, d01.logins, d02.logins, d03.logins, d04.logins, d05.logins, d06.logins,
                    d07.logins, d08.logins, d09.logins, d10.logins, d11.logins, d12.logins,
                    d13.logins, d14.logins, d15.logins, d16.logins, d17.logins, d18.logins,
                    d19.logins, d20.logins, d21.logins, d22.logins, d23.logins, d24.logins,
                    d25.logins, d26.logins, d27.logins, d28.logins, d29.logins, d30.logins,
                )
            },
        )
        .order_by(|(u, ..)| u.name)
        .into()
}

pub fn left_join() -> Query<(), (Text, Nullable<Text>)> {
    users
        .left_join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.name, g.name))
        .order_by(|(u, _)| u.name)
        .into()
}

pub fn left_join_null_propagation() -> Query<(), (Text, Text, Bool)> {
    users
        .left_join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.name, coalesce(g.name, "<none>"), g.id.is_null()))
        .order_by(|(u, _)| u.name)
        .into()
}

pub fn right_join() -> Query<(), (Nullable<Text>, Text)> {
    users
        .right_join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.name, g.name))
        .order_by(|(u, g)| (g.name, u.name))
        .into()
}

pub fn full_join() -> Query<(), (Nullable<Text>, Nullable<Text>)> {
    users
        .full_join(groups, |(u, g)| g.id.eq(u.group_id))
        .select(|(u, g)| (u.name, g.name))
        .order_by(|(u, g)| (g.name, u.name))
        .into()
}

pub fn cross_join() -> Query<(), (Text, Text)> {
    users
        .cross_join(groups)
        .select(|(u, g)| (u.name, g.name))
        .order_by(|(u, g)| (u.name, g.name))
        .into()
}

pub fn natural_join() -> Query<(), (Int8, Text, Text)> {
    user_profiles
        .join(user_settings, |(p, s)| s.user_id.eq(p.user_id))
        .select(|(p, s)| (p.user_id, p.bio, s.theme))
        .order_by(|(p, _)| p.user_id)
        .into()
}

pub fn join_using() -> Query<(), (Int8, Text, Nullable<Text>)> {
    user_profiles
        .left_join(user_settings, |(p, s)| s.user_id.eq(p.user_id))
        .select(|(p, s)| (p.user_id, p.bio, s.theme))
        .order_by(|(p, _)| p.user_id)
        .into()
}

pub fn lateral_first_user_per_group() -> Query<(), (Text, Nullable<Text>)> {
    groups
        .left_join_lateral(
            |g| {
                users
                    .filter(|u| u.group_id.eq(g.id))
                    .select(|u| u.name)
                    .order_by(|u| u.name)
                    .limit(1)
            },
            |_| true,
        )
        .select(|(g, (name,))| (g.name, name))
        .order_by(|(g, _)| g.name)
        .into()
}

pub fn subquery_in_from() -> Query<(), (Text, Int8)> {
    let counts = users
        .group_by(|u| u.group_id)
        .select(|u| (u.group_id, count_star()));
    groups
        .join(counts, |(g, (group_id, _))| group_id.eq(g.id))
        .select(|(g, (_, n))| (g.name, n))
        .order_by(|(g, _)| g.name)
        .into()
}

pub fn tablesample_repeatable() -> Query<(), (Text,)> {
    surus::from(users.tablesample_bernoulli(50).repeatable(42))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn with_ordinality() -> Query<(), (Text, Text, Int8)> {
    users
        .cross_join_lateral(|u| unnest(string_to_array(u.name, "0")).with_ordinality())
        .select(|(u, (part, ord))| (u.name, part, ord))
        .order_by(|(u, (_, ord))| (u.name, ord))
        .into()
}

pub fn set_returning_function_in_from(from: i32, to: i32) -> Query<(), (Text, Int4)> {
    users
        .cross_join(generate_series(from, to))
        .select(|(u, (day,))| (u.name, day))
        .order_by(|(u, (day,))| (u.name, day))
        .into()
}

pub fn rows_from_multiple_functions() -> Query<(), (Text, Nullable<Int4>, Nullable<Text>, Int8)> {
    users
        .cross_join_lateral(|u| {
            rows_from((generate_series(1, 2), unnest(string_to_array(u.name, "0"))))
                .with_ordinality()
        })
        .select(|(u, (n, part, ord))| (u.name, n, part, ord))
        .order_by(|(u, (.., ord))| (u.name, ord))
        .into()
}

pub fn function_columns_in_join_on() -> Query<(Int8, Int8), (Text, Int8)> {
    query(|(start, stop)| {
        surus::from(generate_series(start, stop))
            .join(users, |((n,), u)| u.id.eq(n))
            .select(|((n,), u)| (u.name, n))
            .order_by(|((n,), _)| n)
    })
}

pub fn ordinality_column_in_where() -> Query<(), (Text, Text)> {
    users
        .cross_join_lateral(|u| unnest(string_to_array(u.name, "0")).with_ordinality())
        .filter(|(_, (_, ord))| ord.gt(1))
        .select(|(u, (part, _))| (u.name, part))
        .order_by(|(u, (_, ord))| (u.name, ord))
        .into()
}

pub fn values_in_from_joined() -> Query<(Int8, Int8), (Text, Text)> {
    query(|(first, second)| {
        users
            .join(values((first, "a")).row((second, "b")), |(u, (id, _))| {
                id.eq(u.id)
            })
            .select(|(u, (_, label))| (u.name, label))
            .order_by(|(_, (_, label))| label)
    })
}

pub fn self_join_aliases() -> Query<(), (Text, Text)> {
    users
        .join(users, |(a, b)| a.group_id.eq(b.group_id).and(a.id.lt(b.id)))
        .select(|(a, b)| (a.name, b.name))
        .order_by(|(a, b)| (a.name, b.name))
        .into()
}

pub fn lateral_columns_in_projection_expr() -> Query<(), (Text, Int8)> {
    groups
        .cross_join_lateral(|g| {
            users
                .filter(|u| u.group_id.eq(g.id))
                .select(|_| count_star())
        })
        .select(|(g, (cnt,))| (g.name, cnt + 1))
        .order_by(|(g, _)| g.name)
        .into()
}

pub fn subquery_columns_in_where() -> Query<(), (Text, Int4)> {
    let totals = products.select(|p| (p.name, p.price_cents * p.quantity));
    surus::from(totals)
        .filter(|(_, total)| total.gt(10))
        .select(|(name, total)| (name, total))
        .order_by(|(name, _)| name)
        .into()
}
