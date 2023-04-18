(
SELECT
    u.name results
FROM
    Users u inner join
    (
    SELECT
        user_id, count(user_id) cnt
    FROM
        movierating
    group by user_id
    ) ur
on u.user_id = ur.user_id
order by ur.cnt desc, u.name
limit 1
)
union
(
SELECT
    m.title results
FROM
    Movies m inner join
    (
    SELECT
        movie_id, avg(rating) v
    FROM
        movierating
    where date_format(created_at, "%Y-%m") = "2020-02"
    group by movie_id
    ) mr
on m.movie_id = mr.movie_id
order by mr.v desc, m.title
limit 1
)
