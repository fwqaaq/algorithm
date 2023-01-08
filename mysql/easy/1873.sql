# Write your MySQL query statement below
SELECT
    employee_id,
    IF(
        LEFT(name, 1) != 'M'
        AND employee_id % 2 = 1,
        salary,
        0
    ) bonus
FROM employees
ORDER BY employee_id;