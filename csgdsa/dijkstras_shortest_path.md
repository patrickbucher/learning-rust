---
title: "Dijkstra's Shortst Path Algorithm"
author: "Patrick Bucher"
date: "2024-09-26"
---

# Task

Find the shortest path between two given vertices—_start_ and _finish_—in a
weighted, directed graph.

# Data Structures

Besides graph, three data structures are required:

1. A map of _weights_, with vertices as keys and weights as values.
    - The weight to the _start_ vertex to itself is initialized to zero.
2. A map of _parents_, with vertices as both keys and values.
3. A set of _processed_ vertices.

# Algorithm

The shortest path between _start_ and _finish_ is found as follows:

1. The closest (lowest weight) unprocessed (not an element of _processed_)
vertex is taken from _weights_; it becomes the _current_ vertex, and its weight
the _start weight_.
2. The _current_ vertex' _adjacent_ vertices are processed as follows:
    1. The _new weight_ is computed as the sum of the _start weight_ and the
    weight from the _current_ vertex to the adjacent vertex.
    2. The _adjacent_ vertex and the _new weight_ is inserted into the _weights_
    map; and the _current_ and _adjacent_ vertices are inserted into the
    _parents_ map (key: _adjacent_, value: _current_) under the following
    conditions:
        1. If the _adjacent_ vertex is not in the _weights_ map yet.
        2. If the _adjacent_ vertex has a higher weight in the _weights_ map
        than _new weight_.
3. The _current_ vertex is inserted into the set of _processed_ vertices.
4. Continue with step 1 as long as the _processed_ set is smaller than the set
of all vertices.

# Result

The key-value pairs of the _parents_ map describe the hops to the shortest path
between the _start_ and _finish_ node, from which the shortest path can be
backtracked.
