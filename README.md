# Bloom Filter
A `Bloom Filter` is a set that only stores the hash codes of the items inside of it. It is therefore a probabilistic data structure with the probability of false positives is related to the hashing algorithm used for the data type. Because it only stores the hash code instated of the whole item, it is more memory efficient than other sets.

My implementation provides guaranteed no false negative on checking if a item is part of the set. But it can provide false positives. This makes in unsuitable in situations where not processing a value is unacceptable. However, the probability of this is equal to the probability of a hash collision. So this probability may be acceptable for your application. 

## Design
This wraps a `HashSet` that contains the full hash code for each item added to the set. Because we are only storing the hash codes, we do not take ownership the of the items when we add the to the set and can not give reference to the items.