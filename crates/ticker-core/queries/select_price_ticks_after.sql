SELECT *
FROM price_ticks
WHERE ts >= $1
ORDER BY ts DESC;