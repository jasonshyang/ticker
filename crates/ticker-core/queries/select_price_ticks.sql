SELECT *
FROM price_ticks
WHERE exchange = $1
  AND symbol = $2
ORDER BY ts DESC
LIMIT $3;