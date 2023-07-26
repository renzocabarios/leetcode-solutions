## daily-leads-and-partners

## REFERENCE

- https://leetcode.com/problems/daily-leads-and-partners/

## SOLUTION

``` MySQL
# Write your MySQL query statement below

SELECT date_id, make_name, COUNT(DISTINCT lead_id) AS unique_leads, COUNT(DISTINCT partner_id ) AS unique_partners FROM DailySales GROUP BY date_id, make_name

```


## TIMESTAMP

- 6/05/2023 00:04:18
- 6/06/2023 00:03:32
- 6/10/2023 00:03:36
- 6/15/2023 00:02:02
- 7/26/2023 00:02:28
