# Write your MySQL query statement below
SELECT actor_id, director_id
FROM actorDirector
GROUP BY
    director_id,
    actor_id
HAVING COUNT(*) >= 3;