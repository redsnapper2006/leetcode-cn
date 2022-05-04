select
  t.platform,
  t.experiment_name,
  t.num_experiments
from
  (
    SELECT
      'Android' as platform,
      'Programming' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Programming'
      and platform = 'Android'
    UNION
    SELECT
      'Android' as platform,
      'Sports' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Sports'
      and platform = 'Android'
    UNION
    SELECT
      'Android' as platform,
      'Reading' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Reading'
      and platform = 'Android'
    UNION
    SELECT
      'IOS' as platform,
      'Programming' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Programming'
      and platform = 'IOS'
    UNION
    SELECT
      'IOS' as platform,
      'Sports' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Sports'
      and platform = 'IOS'
    UNION
    SELECT
      'IOS' as platform,
      'Reading' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Reading'
      and platform = 'IOS'
    UNION
    SELECT
      'Web' as platform,
      'Programming',
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Programming'
      and platform = 'Web'
    UNION
    SELECT
      'Web',
      'Sports' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Sports'
      and platform = 'Web'
    UNION
    SELECT
      'Web' as platform,
      'Reading' as experiment_name,
      ifnull(count(experiment_name), 0) as num_experiments
    FROM
      Experiments
    WHERE
      experiment_name = 'Reading'
      and platform = 'Web'
  ) t
