# Write your MySQL query statement below
SELECT
    u.name,
    SUM(t.amount) balance
FROM users u
    INNER JOIN transactions t ON u.account = t.account
GROUP BY u.account
HAVING balance > 10000;