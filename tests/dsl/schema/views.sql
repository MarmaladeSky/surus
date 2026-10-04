DROP VIEW IF EXISTS users_with_groups;
DROP VIEW IF EXISTS grouped_users;

CREATE VIEW users_with_groups AS
    SELECT users.id, users.name, groups.name AS group_name
    FROM users
    JOIN groups ON groups.id = users.group_id;

CREATE VIEW grouped_users AS
    SELECT id, name, email, group_id FROM users WHERE group_id IS NOT NULL
    WITH CHECK OPTION;
