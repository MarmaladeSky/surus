use crate::support::Query;
use schema::*;
use surus::*;

pub fn users_with_group() -> Query {
    users
        .join(groups, groups.id.eq(users.group_id))
        .select((users.name, groups.name))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn group_ancestor_30_levels() -> Query {
    let g0 = alias!(groups, "g0");
    let g1 = alias!(groups, "g1");
    let g2 = alias!(groups, "g2");
    let g3 = alias!(groups, "g3");
    let g4 = alias!(groups, "g4");
    let g5 = alias!(groups, "g5");
    let g6 = alias!(groups, "g6");
    let g7 = alias!(groups, "g7");
    let g8 = alias!(groups, "g8");
    let g9 = alias!(groups, "g9");
    let g10 = alias!(groups, "g10");
    let g11 = alias!(groups, "g11");
    let g12 = alias!(groups, "g12");
    let g13 = alias!(groups, "g13");
    let g14 = alias!(groups, "g14");
    let g15 = alias!(groups, "g15");
    let g16 = alias!(groups, "g16");
    let g17 = alias!(groups, "g17");
    let g18 = alias!(groups, "g18");
    let g19 = alias!(groups, "g19");
    let g20 = alias!(groups, "g20");
    let g21 = alias!(groups, "g21");
    let g22 = alias!(groups, "g22");
    let g23 = alias!(groups, "g23");
    let g24 = alias!(groups, "g24");
    let g25 = alias!(groups, "g25");
    let g26 = alias!(groups, "g26");
    let g27 = alias!(groups, "g27");
    let g28 = alias!(groups, "g28");
    let g29 = alias!(groups, "g29");
    let g30 = alias!(groups, "g30");
    g0.join(g1, g1.id.eq(g0.parent_id))
        .join(g2, g2.id.eq(g1.parent_id))
        .join(g3, g3.id.eq(g2.parent_id))
        .join(g4, g4.id.eq(g3.parent_id))
        .join(g5, g5.id.eq(g4.parent_id))
        .join(g6, g6.id.eq(g5.parent_id))
        .join(g7, g7.id.eq(g6.parent_id))
        .join(g8, g8.id.eq(g7.parent_id))
        .join(g9, g9.id.eq(g8.parent_id))
        .join(g10, g10.id.eq(g9.parent_id))
        .join(g11, g11.id.eq(g10.parent_id))
        .join(g12, g12.id.eq(g11.parent_id))
        .join(g13, g13.id.eq(g12.parent_id))
        .join(g14, g14.id.eq(g13.parent_id))
        .join(g15, g15.id.eq(g14.parent_id))
        .join(g16, g16.id.eq(g15.parent_id))
        .join(g17, g17.id.eq(g16.parent_id))
        .join(g18, g18.id.eq(g17.parent_id))
        .join(g19, g19.id.eq(g18.parent_id))
        .join(g20, g20.id.eq(g19.parent_id))
        .join(g21, g21.id.eq(g20.parent_id))
        .join(g22, g22.id.eq(g21.parent_id))
        .join(g23, g23.id.eq(g22.parent_id))
        .join(g24, g24.id.eq(g23.parent_id))
        .join(g25, g25.id.eq(g24.parent_id))
        .join(g26, g26.id.eq(g25.parent_id))
        .join(g27, g27.id.eq(g26.parent_id))
        .join(g28, g28.id.eq(g27.parent_id))
        .join(g29, g29.id.eq(g28.parent_id))
        .join(g30, g30.id.eq(g29.parent_id))
        .select((g0.name, g30.name))
        .compile()
        .into()
}

pub fn logins_30_day_pivot() -> Query {
    users
        .join(logins_day_01, logins_day_01.user_id.eq(users.id))
        .join(logins_day_02, logins_day_02.user_id.eq(users.id))
        .join(logins_day_03, logins_day_03.user_id.eq(users.id))
        .join(logins_day_04, logins_day_04.user_id.eq(users.id))
        .join(logins_day_05, logins_day_05.user_id.eq(users.id))
        .join(logins_day_06, logins_day_06.user_id.eq(users.id))
        .join(logins_day_07, logins_day_07.user_id.eq(users.id))
        .join(logins_day_08, logins_day_08.user_id.eq(users.id))
        .join(logins_day_09, logins_day_09.user_id.eq(users.id))
        .join(logins_day_10, logins_day_10.user_id.eq(users.id))
        .join(logins_day_11, logins_day_11.user_id.eq(users.id))
        .join(logins_day_12, logins_day_12.user_id.eq(users.id))
        .join(logins_day_13, logins_day_13.user_id.eq(users.id))
        .join(logins_day_14, logins_day_14.user_id.eq(users.id))
        .join(logins_day_15, logins_day_15.user_id.eq(users.id))
        .join(logins_day_16, logins_day_16.user_id.eq(users.id))
        .join(logins_day_17, logins_day_17.user_id.eq(users.id))
        .join(logins_day_18, logins_day_18.user_id.eq(users.id))
        .join(logins_day_19, logins_day_19.user_id.eq(users.id))
        .join(logins_day_20, logins_day_20.user_id.eq(users.id))
        .join(logins_day_21, logins_day_21.user_id.eq(users.id))
        .join(logins_day_22, logins_day_22.user_id.eq(users.id))
        .join(logins_day_23, logins_day_23.user_id.eq(users.id))
        .join(logins_day_24, logins_day_24.user_id.eq(users.id))
        .join(logins_day_25, logins_day_25.user_id.eq(users.id))
        .join(logins_day_26, logins_day_26.user_id.eq(users.id))
        .join(logins_day_27, logins_day_27.user_id.eq(users.id))
        .join(logins_day_28, logins_day_28.user_id.eq(users.id))
        .join(logins_day_29, logins_day_29.user_id.eq(users.id))
        .join(logins_day_30, logins_day_30.user_id.eq(users.id))
        .select((
            users.name,
            logins_day_01.logins,
            logins_day_02.logins,
            logins_day_03.logins,
            logins_day_04.logins,
            logins_day_05.logins,
            logins_day_06.logins,
            logins_day_07.logins,
            logins_day_08.logins,
            logins_day_09.logins,
            logins_day_10.logins,
            logins_day_11.logins,
            logins_day_12.logins,
            logins_day_13.logins,
            logins_day_14.logins,
            logins_day_15.logins,
            logins_day_16.logins,
            logins_day_17.logins,
            logins_day_18.logins,
            logins_day_19.logins,
            logins_day_20.logins,
            logins_day_21.logins,
            logins_day_22.logins,
            logins_day_23.logins,
            logins_day_24.logins,
            logins_day_25.logins,
            logins_day_26.logins,
            logins_day_27.logins,
            logins_day_28.logins,
            logins_day_29.logins,
            logins_day_30.logins,
        ))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn left_join() -> Query {
    let g = groups.nullable();
    users
        .left_join(g, g.id.eq(users.group_id))
        .select((users.name, g.name))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn left_join_null_propagation() -> Query {
    let g = groups.nullable();
    users
        .left_join(g, g.id.eq(users.group_id))
        .select((users.name, coalesce(g.name, "<none>"), g.id.is_null()))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn right_join() -> Query {
    let u = users.nullable();
    u.right_join(groups, groups.id.eq(u.group_id))
        .select((u.name, groups.name))
        .order_by((groups.name, u.name))
        .compile()
        .into()
}

pub fn full_join() -> Query {
    let (u, g) = (users.nullable(), groups.nullable());
    u.full_join(g, g.id.eq(u.group_id))
        .select((u.name, g.name))
        .order_by((g.name, u.name))
        .compile()
        .into()
}

pub fn cross_join() -> Query {
    users
        .cross_join(groups)
        .select((users.name, groups.name))
        .order_by((users.name, groups.name))
        .compile()
        .into()
}

pub fn natural_join() -> Query {
    user_profiles
        .natural_join(user_settings)
        .select((
            user_profiles.user_id,
            user_profiles.bio,
            user_settings.theme,
        ))
        .order_by(user_profiles.user_id)
        .compile()
        .into()
}

pub fn join_using() -> Query {
    let settings = user_settings.nullable();
    user_profiles
        .left_join_using(settings, settings.user_id)
        .select((user_profiles.user_id, user_profiles.bio, settings.theme))
        .order_by(user_profiles.user_id)
        .compile()
        .into()
}

pub fn lateral_first_user_per_group() -> Query {
    let g = alias!(groups, "g");
    let first = users
        .select(users.name)
        .filter(users.group_id.eq(g.id))
        .order_by(users.name)
        .limit(1);
    let u = lateral(alias!(first, "u")).nullable();
    let (name,) = u.columns();
    g.left_join(u, true)
        .select((g.name, name))
        .order_by(g.name)
        .compile()
        .into()
}

pub fn subquery_in_from() -> Query {
    let g = alias!(groups, "g");
    let counts = users
        .select((users.group_id, count_star().as_("n")))
        .group_by(users.group_id);
    let c = alias!(counts, "c");
    let (group_id, n) = c.columns();
    g.join(c, group_id.eq(g.id))
        .select((g.name, n))
        .order_by(g.name)
        .compile()
        .into()
}

pub fn tablesample_repeatable() -> Query {
    users
        .tablesample_bernoulli(50f32)
        .repeatable(42.0)
        .select(users.name)
        .order_by(users.name)
        .compile()
        .into()
}

pub fn with_ordinality() -> Query {
    let u = alias!(users, "u");
    let w = alias!(unnest(string_to_array(u.name, "0")).with_ordinality(), "w");
    let (part, ord) = w.columns();
    u.cross_join(w)
        .select((u.name, part, ord))
        .order_by((u.name, ord))
        .compile()
        .into()
}

pub fn set_returning_function_in_from(from: i32, to: i32) -> Query {
    let s = generate_series(from, to);
    let (day,) = s.columns();
    users
        .cross_join(s)
        .select((users.name, day))
        .order_by((users.name, day))
        .compile()
        .into()
}

pub fn rows_from_multiple_functions() -> Query {
    let u = alias!(users, "u");
    let functions = rows_from((generate_series(1, 2), unnest(string_to_array(u.name, "0"))));
    let r = alias!(functions.with_ordinality(), "r");
    let (n, part, ord) = r.columns();
    u.cross_join(r)
        .select((u.name, n, part, ord))
        .order_by((u.name, ord))
        .compile()
        .into()
}

pub fn function_columns_in_join_on(from: i64, to: i64) -> Query {
    let s = generate_series(from, to);
    let (n,) = s.columns();
    let u = alias!(users, "u");
    s.join(u, u.id.eq(n))
        .select((u.name, n))
        .order_by(n)
        .compile()
        .into()
}

pub fn ordinality_column_in_where() -> Query {
    let u = alias!(users, "u");
    let w = alias!(unnest(string_to_array(u.name, "0")).with_ordinality(), "w");
    let (part, ord) = w.columns();
    u.cross_join(w)
        .select((u.name, part))
        .filter(ord.gt(1))
        .order_by((u.name, ord))
        .compile()
        .into()
}

pub fn values_in_from_joined(first: i64, second: i64) -> Query {
    let u = alias!(users, "u");
    let v = alias!(values([(first, "a"), (second, "b")]), "v");
    let (id, label) = v.columns();
    u.join(v, id.eq(u.id))
        .select((u.name, label))
        .order_by(label)
        .compile()
        .into()
}

pub fn self_join_aliases() -> Query {
    let a = alias!(users, "a");
    let b = alias!(users, "b");
    a.join(b, a.group_id.eq(b.group_id).and(a.id.lt(b.id)))
        .select((a.name, b.name))
        .order_by((a.name, b.name))
        .compile()
        .into()
}

pub fn lateral_columns_in_projection_expr() -> Query {
    let g = alias!(groups, "g");
    let u = alias!(users, "u");
    let l = lateral(alias!(
        u.select(count_star().as_("cnt"))
            .filter(u.group_id.eq(g.id)),
        "l"
    ));
    let (cnt,) = l.columns();
    g.cross_join(l)
        .select((g.name, cnt + 1))
        .order_by(g.name)
        .compile()
        .into()
}

pub fn subquery_columns_in_where() -> Query {
    let totals = products.select((
        products.name,
        (products.price_cents * products.quantity).as_("total"),
    ));
    let s = alias!(totals, "s");
    let (name, total) = s.columns();
    s.select((name, total))
        .filter(total.gt(10))
        .order_by(name)
        .compile()
        .into()
}
