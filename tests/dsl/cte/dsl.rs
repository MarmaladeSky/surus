use crate::schema::{groups, products, tickets, users};
use surus::prelude::*;
use surus::types::{Array, Int4, Text};
use surus::{Query, array, count_star, cte, from, query, recursive};

pub fn named_users() -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        let named = cte(users.filter(|u| u.name.eq(name)).select(|u| (u.id, u.name)));
        from(named).select(|(_, name)| name)
    })
}

pub fn recursive_ancestors() -> Query<(Text,), (Text, Int4)> {
    query(|(name,)| {
        let ancestors = recursive(
            groups
                .filter(|g| g.name.eq(name))
                .select(|g| (g.id, g.name, g.parent_id, 0)),
            |ancestors| {
                groups
                    .join(ancestors, |(g, (_, _, parent_id, _))| g.id.eq(parent_id))
                    .select(|(g, (.., depth))| (g.id, g.name, g.parent_id, depth + 1))
            },
        );
        from(ancestors)
            .select(|(_, name, _, depth)| (name, depth))
            .order_by(|(.., depth)| depth)
    })
}

pub fn recursive_descendants_with_path() -> Query<(Text,), (Array<Text>,)> {
    query(|(name,)| {
        let tree = recursive(
            groups
                .filter(|g| g.name.eq(name))
                .select(|g| (g.id, array((g.name,)))),
            |tree| {
                groups
                    .join(tree, |(g, (id, _))| g.parent_id.eq(id))
                    .select(|(g, (_, path))| (g.id, path.concat(g.name)))
            },
        );
        from(tree)
            .select(|(_, path)| path)
            .order_by(|(_, path)| path)
    })
}

pub fn data_modifying() -> Query<(Text, Text), (Text, Text)> {
    query(|(group, batch)| {
        let group_id = groups.filter(|g| g.name.eq(group)).select(|g| g.id);
        let archived = cte(users
            .filter(|u| u.group_id.eq(group_id))
            .delete()
            .returning(|u| u.name));
        tickets
            .insert_from(from(archived).order_by(|(name,)| name), |row, (name,)| {
                row.batch(batch).name(name)
            })
            .returning(|t| (t.batch, t.name))
    })
}

pub fn chained_materialized() -> Query<(), (Text, Text)> {
    let roots = cte(groups
        .filter(|g| g.parent_id.is_null())
        .select(|g| (g.id, g.name)))
    .materialized();
    let children = cte(groups
        .join(roots, |(g, (id, _))| g.parent_id.eq(id))
        .select(|(g, (_, root))| (g.name, root)))
    .not_materialized();
    from(children)
        .select(|(name, root)| (name, root))
        .order_by(|(name, _)| name)
        .into()
}

pub fn cte_column_expressions() -> Query<(), (Text, Int4)> {
    let totals = cte(products.select(|p| (p.name, p.price_cents * p.quantity)));
    from(totals)
        .filter(|(_, total)| total.gt(10))
        .select(|(name, total)| (name, total + 1))
        .order_by(|(name, _)| name)
        .into()
}

pub fn cte_joined_to_table() -> Query<(), (Text,)> {
    let big = cte(users
        .group_by(|u| u.group_id)
        .having(|_| count_star().gt(1))
        .select(|u| u.group_id));
    groups
        .join(big, |(g, (group_id,))| group_id.eq(g.id))
        .select(|(g, _)| g.name)
        .order_by(|(g, _)| g.name)
        .into()
}

pub fn cte_referenced_twice() -> Query<(), (Text, Text)> {
    let named = cte(users.select(|u| (u.id, u.name)));
    from(named.clone())
        .join(named, |((a_id, _), (b_id, _))| a_id.eq(b_id))
        .select(|((_, a), (_, b))| (a, b))
        .order_by(|((_, a), _)| a)
        .into()
}

pub fn cte_in_subquery() -> Query<(), (Text,)> {
    let with_email = cte(users.filter(|u| u.email.is_not_null()).select(|u| u.id));
    users
        .filter(|u| u.id.is_in(from(with_email).select(|(id,)| id)))
        .select(|u| u.name)
        .order_by(|u| u.name)
        .into()
}

pub fn recursive_depth_expression() -> Query<(), (Text, Int4)> {
    let tree = recursive(
        groups
            .filter(|g| g.parent_id.is_null())
            .select(|g| (g.id, g.name, 0)),
        |tree| {
            groups
                .join(tree, |(g, (id, ..))| g.parent_id.eq(id))
                .select(|(g, (.., depth))| (g.id, g.name, depth + 1))
        },
    );
    from(tree)
        .select(|(_, name, depth)| (name, depth * 10))
        .order_by(|(_, name, depth)| (depth, name))
        .into()
}

pub fn modifying_cte_returning_used() -> Query<(Text,), (Text,)> {
    query(|(name,)| {
        let inserted = cte(groups
            .insert(|g| g.name(name))
            .returning(|g| (g.id, g.name)));
        from(inserted).select(|(_, name)| name.concat("!"))
    })
}
