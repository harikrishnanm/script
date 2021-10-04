local charset = {}  do -- [0-9a-zA-Z]
  for c = 48, 57  do table.insert(charset, string.char(c)) end
  for c = 65, 90  do table.insert(charset, string.char(c)) end
  for c = 97, 122 do table.insert(charset, string.char(c)) end
end

local function randomString(length)
  if not length or length <= 0 then return '' end
  math.randomseed(os.clock()^5)
  return randomString(length - 1) .. charset[math.random(1, #charset)]
end

wrk.method = "GET"
-- wrk.body   = '{"raw_data":" .. randomString(15) .. ","alg":"AES_GCM"}'
--wrk.headers["Content-Type"] = "application/json"
wrk.headers["Authorization"] = "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJPbmxpbmUgSldUIEJ1aWxkZXIiLCJpYXQiOjE2MzI0ODYwOTAsImV4cCI6MTY2NDAyMjA5MCwiYXVkIjoid3d3LmV4YW1wbGUuY29tIiwic3ViIjoianJvY2tldEBleGFtcGxlLmNvbSIsInJvbGVzIjpbIlNJVEUgQURNSU4iLCJBRE1JTiJdLCJ1c2VyIjoic2l0ZWFkbWluIn0.DKfqDTG0VTvdrNhhgHUDJsMHjaL7lMrF395oTbbLhkI"
response = function(status, headers, body)
    if status ~= 200 then
        io.write("------------------------------\n")
        io.write("Response with status: ".. status .."\n")
        io.write("------------------------------\n")
        io.write("[response] Body:\n")
        io.write(body .. "\n")
    end
end

-- done = function(summary, latency, requests)
--    io.write("------------------------------\n")
--   for _, p in pairs({ 10, 20, 30, 40,  50, 75, 90, 95, 99, 99.999 }) do
--      n = latency:percentile(p)
--      io.write(string.format("%g%%,%d\n", p, n))
--   end
--end
