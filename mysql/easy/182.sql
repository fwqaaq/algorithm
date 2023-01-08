# Write your MySQL query statement below
# SELECT email FROM Person GROUP BY email HAVING COUNT(*) > 1;

SELECT email
FROM (
        SELECT
            email,
            COUNT(email) num
        FROM Person
        GROUP BY email
    ) a
WHERE a.num > 1;