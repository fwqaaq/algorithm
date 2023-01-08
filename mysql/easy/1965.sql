# Write your MySQL query statement below
SELECT t.employee_id
FROM (
        SELECT employee_id
        FROM employees
        UNION ALL
        SELECT employee_id
        FROM salaries
    ) t
GROUP BY t.employee_id
HAVING COUNT(*) = 1
ORDER BY t.employee_id;