
class Blergh(object):
    def __init__(self, value):
        self.value = value

    def filter_val(self):
        print(f"Filter {self.value}")
        return self.value - 1 > 0

    def map_val(self):
        print(f"Map {self.value}")
        return self.value - 1


nlist = [Blergh(i) for i in range(3)]


print([x.map_val() for x in nlist if x.filter_val()])

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
