# lapapo-design
alternative design for a decentralised network

- network is made up of nodes
- each node is identified by unique assymetric keypair, they share the public key but private key they keep secret
- a nodes publicly shared information is: node id (public key), IP address, UDP port, info counter, info signature (highest info counter is most recent)
- up to certain network size each node stores info of all other nodes on network
- once network above certain size (1000s-10,000s nodes) each node starts using an algorithm to determine which other nodes to store info on


algorithm is as follows:

- node IDs are byte length L, all must be same length, relies on node IDs being randomly distributed to work effectively, this should be true for randomly generated asymmetric keypairs
- xor the other node id with own node id and get trailing zeroes of result
- calculate log2 absolute difference between own node id and other node id (as if they were numbers), absolute difference means loops around so 0 and max are only 1 away from each other instead of max
- select number N such that selecting only nodes with trailing zeroes >= 2^N totals >= 2^N nodes and selecting only nodes with log2 absolute difference < 2^(L-N) totals >= 2^N nodes aswell
- start N at 0 and increase until find maximum value of N for which this is true in both cases
- select this value of N and only store info on nodes that meet the criteria for either the trailing zeroes or log2 absolute difference case
- each time a new nodes info is added can see whether it is possible to increase N by 1 and still have at least 2^N nodes totalled in both cases in which case can increase and remove info of all nodes that met lower value of N but not now N+1.
- each time remove a nodes info, see if both cases still total minimum of 2^N nodes, if either don't then decrease N by 1

- this method should approximate total number of nodes on network being 2^(N*2), each node is storing approx 2^N * 2 (log2 abs diff and trailing) other nodes info which is approx (2 * ✓(total nodes)) so scales with square root of network size
- for example if total number of nodes is 2^30 (≈1 billion) then each node would store approx 2^15 * 2 = 2^16 ≈ 65,000 other nodes info


using this algorithm should make the hops required to find info on another user low (<4)

- get node id want to get info of
- of all nodes have info of, find one which is closest by log2 absolute difference to target node (lowest value)
- send request to that node for info on target node info
- they either send back the info if they know or send back info on closest node they know to target node
- repeat request with this node etc until responds with target node info
- if reach "dead end" where get no response then can back track and request closest node info to target excluding previously tried one
- effectively depth first search

if you simulate this setup then only takes few hops to find another nodes info most of the time, higher hop counts decreases asympotically so get majority low down and occasional high ones. hops average does not change with size of the network

this is better than current decentralised network standards such as kademilia which take on average log2(network size) hops to find another nodes info