SELECT query_name, ROUND(SUM(IF(rating < 3, 1, 0))*100/COUNT(*), 2) AS quality, ROUND(AVG(rating/position), 2) AS poor_query_percentage
FROM queries
GROUP BY query_name;
