SELECT
  l.date_id,
  l.make_name,
  l.unique_leads,
  p.unique_partners
FROM
  (
    SELECT
      dsl.date_id,
      dsl.make_name,
      COUNT(*) unique_leads
    FROM
      (
        SELECT
          DISTINCT date_id,
          make_name,
          lead_id
        FROM
          DailySales
      ) dsl
    GROUP BY
      dsl.date_id,
      dsl.make_name
  ) l,
  (
    SELECT
      dsp.date_id,
      dsp.make_name,
      COUNT(*) unique_partners
    FROM
      (
        SELECT
          DISTINCT date_id,
          make_name,
          partner_id
        FROM
          DailySales
      ) dsp
    GROUP BY
      dsp.date_id,
      dsp.make_name
  ) p
WHERE
  p.date_id = l.date_id
  AND p.make_name = l.make_name
