# Write your MySQL query statement below
SELECT
    user_id,
    CONCAT(
        UPPER(LEFT(name, 1)),
        lower(substr(name, 2))
    ) as name
FROM users
ORDER BY user_id;