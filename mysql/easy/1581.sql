# Write your MySQL query statement below
# SELECT v.customer_id, COUNT(*) count_no_trans FROM visits v WHERE v.visit_id NOT IN (
#     SELECT visit_id FROM transactions
# ) GROUP BY v.customer_id;

SELECT
    v.customer_id,
    COUNT(*) count_no_trans
FROM visits v
    LEFT JOIN transactions t ON v.visit_id = t.visit_id
WHERE t.visit_id IS NULL
GROUP BY v.customer_id;