#Dns
- Dns uses UDP (although it can use TCP as well) with a packet size of 512 bytes.
$| Section | size | Type | purpose |
| --------------- | --------------- | --------------- | --------------- |
| Header | 12 bytes | Header | information about the query/response |
| Question | variable | List of questions | domain in question and the requested record type |
| Answer | variable | list of records | requested records |
| Authority | Variable | list of records | a list of name servers to resolve queries recursively |



