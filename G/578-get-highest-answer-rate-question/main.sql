SELECT aa.question_id AS survey_log
FROM (
    SELECT ((SELECT COUNT(*)
             FROM SurveyLog b
             WHERE b.question_id = t.question_id
               AND b.action = 'answer')/t.answer_cnt) result,
           t.question_id
    FROM (
        SELECT a.question_id, COUNT(*) answer_cnt
        FROM SurveyLog a
        WHERE a.action = 'show'
        GROUP BY a.question_id
    ) t
    ORDER BY result DESC, t.question_id ASC
    limit 1
) aa
