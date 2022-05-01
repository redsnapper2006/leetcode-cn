select
  p.id,
  (
    case
      when IFnull(p.parent, 'parentnull') = 'parentnull' then 'Root'
      when p.parent != 'parentnull'
      and p.children != 'childrennull' then 'Inner'
      else 'Leaf'
    end
  ) as type
FROM
  (
    SELECT
      t.id,
      t.p_id as parent,
      ifnull(
        (
          select
            distinct t.id
          from
            tree t2
          where
            t.id = ifnull(t2.p_id, "null")
        ),
        'childrennull'
      ) as children
    FROM
      tree t
  ) p
