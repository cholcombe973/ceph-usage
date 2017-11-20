# ceph-usage
Gather ceph usage information about the cluster.  This usage information is made available
over tcp with protobuf serialization.  Check out the [api] for more details on the protocol.

## Communication:
```
                  Hello
        +----------------------------^+---------------+
        |                             | Server A:8888 |
+-------+-+                           +---------------+
|  Client |       ClusterUsage             |
+---+--+--+^-------------------------------+
    ^  |          Hello                +--------------+
    |  +------------------------------^+Server B:8888 |
    |                                  +-----+--------+
    |             ClusterUsage               |
    +----------------------------------------+

```
The communication is a Server/Client model where the Server is run on a Ceph Mon and the
Client can be run anywhere that has network access.

[api]: https://github.com/cholcombe973/ceph-usage/blob/master/api/protos/service.proto
