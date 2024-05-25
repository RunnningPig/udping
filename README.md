udping
------------
Send UDP ping packets to network hosts 

## How to use

ping mode

```shell
$ udping 127.0.0.1
UDPING 127.0.0.1:23333
Pong from 127.0.0.1:23333: seq=1 time=0.154 ms
Pong from 127.0.0.1:23333: seq=2 time=0.144 ms
Pong from 127.0.0.1:23333: seq=3 time=0.122 ms
Pong from 127.0.0.1:23333: seq=4 time=0.144 ms
Pong from 127.0.0.1:23333: seq=5 time=0.126 ms
```

pong mode

```shell
$ udping --server
Listen on 0.0.0.0:23333
Ping from 127.0.0.1:55673: seq=1
Ping from 127.0.0.1:55673: seq=2
Ping from 127.0.0.1:55673: seq=3
Ping from 127.0.0.1:55673: seq=4
Ping from 127.0.0.1:55673: seq=5
```