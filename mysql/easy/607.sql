# Write your MySQL query statement below
SELECT s.name
FROM salesPerson s
WHERE s.sales_id NOT IN (
        SELECT o.sales_id
        FROM Orders o
            LEFT JOIN Company c ON o.com_id = c.com_id
        WHERE c.name = 'RED'
    );