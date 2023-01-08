-- SELECT (

--         SELECT DISTINCT salary

--         FROM Employee

--         ORDER BY salary DESC

--         LIMIT 1

--         OFFSET

--             1

--     ) SecondHighestSalary;

SELECT
    max(salary) SecondHighestSalary
FROM Employee
WHERE salary < (
        SELECT max(salary)
        FROM Employee
    );