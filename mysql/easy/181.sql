# Write your MySQL query statement below
# SELECT e1.name Employee FROM Employee e1, Employee e2 WHERE e2.id = e1.managerId AND e1.salary > e2.salary;

SELECT e1.name Employee
FROM Employee e1
    LEFT JOIN Employee e2 ON e1.managerId = e2.id
WHERE e1.salary > e2.salary;