# Write your MySQL query statement below
# SELECT c.name Customers FROM Customers c LEFT JOIN Orders o ON c.id = o.CustomerId WHERE o.CustomerId IS NULL;

SELECT name Customers
FROM Customers
WHERE id NOT IN (
        SELECT CustomerId
        FROM Orders
    );