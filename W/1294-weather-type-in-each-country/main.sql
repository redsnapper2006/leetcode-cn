select
    c.country_name,
    CASE
        WHEN AVG(w.weather_state) <= 15 THEN "Cold"
        WHEN AVG(w.weather_state) >= 25 THEN "Hot"
        ELSE "Warm"
    END as weather_type
from
    Countries c,
    Weather w
where
    w.country_id = c.country_id
    and DATE_FORMAT(w.day, "%Y-%m") = '2019-11'
GROUP BY
    w.country_id
