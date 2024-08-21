import time

SLEEP_TIME = 0
HOW_MANY = 10000

# Another example that bears interesting results
# SLEEP_TIME = .05
# HOW_MANY = 10

class Blergh(object):
    def __init__(self, value):
        self.value = value
        self.cached_mapped_value = None

    def filter_val(self):
        #print(f"Filter {self.value}")
        time.sleep(SLEEP_TIME)
        return self.value - 1 > 0

    def map_val(self):
        if self.cached_mapped_value is not None:
            #print(f"Map (cached) {self.value}")
            return self.cached_mapped_value
        else:
            #print(f"Map {self.value}")
            time.sleep(SLEEP_TIME)
            return self.value - 1

    def filter_val_faster(self):
        #print(f"Map {self.value}")
        time.sleep(SLEEP_TIME)
        self.cached_mapped_value = self.value - 1
        return self.cached_mapped_value



nlist = [Blergh(i) for i in range(1, HOW_MANY+1)]

no_cache_start = time.time()
print("No caching")
cacheless_list = [x.map_val() for x in nlist if x.filter_val()]
if len(cacheless_list) > 5:
    print(cacheless_list[:5])
print("Time taken: ", time.time() - no_cache_start)

cache_start = time.time()
print("Now with caching")
cache_list = [x.map_val() for x in nlist if x.filter_val_faster()]
if len(cache_list) > 5:
    print(cache_list[:5])
print("Time taken: ", time.time() - cache_start)

# With HOW_MANY set to 3, this will print:
# This will print:
# Filter 1
# Filter 2
# Map 2
# Filter 3
# Map 3
# [1, 2]

# Since the underlying operations is the same as my simple list of numbers
# example, you can see that we are needlessly calling that function twice for
# each element (at least, each element that passes the filter).
# If the self.value - 1 operation is expensive, that would be signifigantly slower.
# Adding in the time.sleep(1) simulates this, makes it seem like each of those operations takes 1 second

# You could get around this by restructuring your code, or doing some caching or the likes.

# I included a filter_val_faster method that caches the mapped value, so you don't have to recalculate it.
# You can see by running this the performance implications of each. You can
# play with the sleep time or the range to see how it affects performance.

# With 10,000 elements, you can see that (on this machine), the non-cached version is twice as slow, and that's WITHOUT the sleep time.

# Reviewed and acknowledged; still understand zero of the above at this point in time.