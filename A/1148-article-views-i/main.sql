SELECT
  DISTINCT va.author_id as id
FROM
  Views va,
  Views vv
WHERE
  va.author_id = vv.viewer_id
  AND va.article_id = vv.article_id
ORDER BY
  va.author_id
