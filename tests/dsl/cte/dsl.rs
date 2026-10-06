use crate::support::Query;
use schema::*;
use surus::*;

pub fn named_users(name: &str) -> Query {
    let named = cte(
        "named",
        users
            .select((users.id, users.name))
            .filter(users.name.eq(name)),
    );
    let (_, name) = named.columns();
    named.select(name).compile().into()
}

pub fn recursive_ancestors(leaf: &str) -> Query {
    let start = groups
        .select((
            groups.id,
            groups.name,
            groups.parent_id,
            param(0).as_("depth"),
        ))
        .filter(groups.name.eq(leaf));
    let ancestors = recursive_cte("ancestors", start, |ancestors| {
        let (_, _, parent_id, depth) = ancestors.columns();
        groups.join(ancestors, groups.id.eq(parent_id)).select((
            groups.id,
            groups.name,
            groups.parent_id,
            depth + 1,
        ))
    });
    let (_, name, _, depth) = ancestors.columns();
    ancestors
        .select((name, depth))
        .order_by(depth)
        .compile()
        .into()
}

pub fn recursive_descendants_with_path(root: &str) -> Query {
    let start = groups
        .select((groups.id, array((groups.name,)).as_("path")))
        .filter(groups.name.eq(root));
    let tree = recursive_cte("tree", start, |tree| {
        let (id, path) = tree.columns();
        groups
            .join(tree, groups.parent_id.eq(id))
            .select((groups.id, path.concat(groups.name)))
    });
    let (_, path) = tree.columns();
    tree.select(path).order_by(path).compile().into()
}

pub fn data_modifying(group: &str, batch: &str) -> Query {
    let group_id = groups.select(groups.id).filter(groups.name.eq(group));
    let archived = cte(
        "archived",
        users
            .delete()
            .filter(users.group_id.eq(group_id))
            .returning(users.name),
    );
    let (name,) = archived.columns();
    tickets
        .insert((tickets.batch, tickets.name))
        .select(archived.select((param(batch), name)).order_by(name))
        .returning((tickets.batch, tickets.name))
        .compile()
        .into()
}

pub fn chained_materialized() -> Query {
    let roots = cte(
        "roots",
        groups
            .select((groups.id, groups.name))
            .filter(groups.parent_id.is_null()),
    )
    .materialized();
    let (root_id, root_name) = roots.columns();
    let g = alias!(groups, "g");
    let children = g
        .join(roots, g.parent_id.eq(root_id))
        .select((g.name, root_name.as_("root")));
    let children = cte("children", children).not_materialized();
    let (name, root) = children.columns();
    children
        .select((name, root))
        .order_by(name)
        .compile()
        .into()
}

pub fn cte_column_expressions() -> Query {
    let t = cte(
        "t",
        products.select((
            products.name,
            (products.price_cents * products.quantity).as_("total"),
        )),
    );
    let (name, total) = t.columns();
    t.select((name, total + 1))
        .filter(total.gt(10))
        .order_by(name)
        .compile()
        .into()
}

pub fn cte_joined_to_table() -> Query {
    let big = cte(
        "big",
        users
            .select(users.group_id)
            .group_by(users.group_id)
            .having(count_star().gt(1)),
    );
    let (group_id,) = big.columns();
    let g = alias!(groups, "g");
    g.join(big, group_id.eq(g.id))
        .select(g.name)
        .order_by(g.name)
        .compile()
        .into()
}

pub fn cte_referenced_twice() -> Query {
    let t = cte("t", users.select((users.id, users.name)));
    let (a, b) = (alias!(t.clone(), "a"), alias!(t, "b"));
    let ((a_id, a_name), (b_id, b_name)) = (a.columns(), b.columns());
    a.join(b, a_id.eq(b_id))
        .select((a_name, b_name))
        .order_by(a_name)
        .compile()
        .into()
}

pub fn cte_in_subquery() -> Query {
    let t = cte(
        "t",
        users.select(users.id).filter(users.email.is_not_null()),
    );
    let (id,) = t.columns();
    users
        .select(users.name)
        .filter(users.id.in_(t.select(id)))
        .order_by(users.name)
        .compile()
        .into()
}

pub fn recursive_depth_expression() -> Query {
    let roots = groups
        .select((groups.id, groups.name, param(0).as_("depth")))
        .filter(groups.parent_id.is_null());
    let tree = recursive_cte("tree", roots, |tree| {
        let (id, _, depth) = tree.columns();
        groups
            .join(tree, groups.parent_id.eq(id))
            .select((groups.id, groups.name, depth + 1))
    });
    let (_, name, depth) = tree.columns();
    tree.select((name, depth * 10))
        .order_by((depth, name))
        .compile()
        .into()
}

pub fn modifying_cte_returning_used(name: &str) -> Query {
    let ins = cte(
        "ins",
        groups
            .insert((groups.name,))
            .values((name,))
            .returning((groups.id, groups.name)),
    );
    let (_, name) = ins.columns();
    ins.select(name.concat("!")).compile().into()
}
