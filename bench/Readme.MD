Check the x-auth-token header in the encrypt.lua script

Then run the benchmark using the following command

`wrk -c10 -t8 -d60s -s ./encrypt.lua --latency http://localhost:8088/encrypt/v1`
