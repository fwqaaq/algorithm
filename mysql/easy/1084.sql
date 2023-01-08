# Write your MySQL query statement below
# SELECT product_id, product_name FROM product WHERE product_id IN (
#     SELECT product_id FROM sales GROUP BY product_id HAVING MAX(sale_date) <= '2019-03-31' AND MIN(sale_date) >= '2019-01-01'
# );

SELECT
    p.product_id,
    p.product_name
FROM product p
    INNER JOIN sales s ON p.product_id = s.product_id
GROUP BY s.product_id
HAVING
    MAX(sale_date) <= '2019-03-31'
    AND MIN(sale_date) >= '2019-01-01';