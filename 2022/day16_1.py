# Advent of code 2022, by Ghostkeeper

import functools
import get_input_file
import queue
import re

# Construct a graph from the input.
text = open(get_input_file.get_path(16, 1)).read()
# First map all vertices once.
vertex_labels = []
for match in re.finditer(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (([A-Z]{2},? ?)+)", text):
	vertex_labels.append((int(match.group(2)), match.group(1)))
vertex_labels = [label for weight, label in reversed(sorted(vertex_labels))]  # Sort by flow rate, so that indices don't change if we remove flow=0 valves.

# Connect them up.
adjacent = []
flow_rate = []
for i in range(len(vertex_labels)):
	adjacent.append([])
	flow_rate.append(-1)
for match in re.finditer(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (([A-Z]{2},? ?)+)", text):
	vertex = vertex_labels.index(match.group(1))
	flow_rate[vertex] = int(match.group(2))
	connections = match.group(3).split(", ")
	for connection in connections:
		adjacent[vertex].append((vertex_labels.index(connection), 1))  # Until we collapse vertices, every edge takes 1 minute to traverse.

# Put the AA vertex at the end (which would otherwise not be relevant.
start_vertex = vertex_labels.index("AA")
num_relevant = flow_rate.index(0)
if flow_rate[start_vertex] == 0:
	adjacent[start_vertex], adjacent[num_relevant] = adjacent[num_relevant], adjacent[start_vertex]
	flow_rate[start_vertex], flow_rate[num_relevant] = flow_rate[num_relevant], flow_rate[start_vertex]
	vertex_labels[start_vertex], vertex_labels[num_relevant] = vertex_labels[num_relevant], vertex_labels[start_vertex]

	for edge_list in adjacent:  # Swap connections to start and last-relevant as well.
		to_start = -1
		to_num = -1
		for i, edge in enumerate(edge_list):
			if edge[0] == start_vertex:
				to_start = i
			if edge[0] == num_relevant:
				to_num = i
		if to_start != -1:
			edge_to_start = (num_relevant, edge_list[to_start][1])
			edge_list[to_start] = edge_to_start
		if to_num != -1:
			edge_to_num = (start_vertex, edge_list[to_num][1])
			edge_list[to_num] = edge_to_num
	num_relevant += 1

# Add an edge from every relevant vertex to every other relevant vertex along the shortest route, creating a complete graph with as many shortcuts as possible.
clique = [[] for vertex in adjacent]
for start_vertex in range(len(adjacent)):
	shortest_distance = [float("inf") for vertex in range(len(adjacent))]
	shortest_distance[start_vertex] = 0
	todo = queue.PriorityQueue()
	for i in range(len(adjacent)):
		todo.put((shortest_distance[i], i))
	open_set = {start_vertex}
	while open_set:
		distance, next = todo.get()
		open_set.remove(next)

		for neighbour in adjacent[next]:
			neighbour_dist = shortest_distance[next] + 1
			if neighbour_dist < shortest_distance[neighbour[0]]:
				shortest_distance[neighbour[0]] = neighbour_dist
				if neighbour[0] not in open_set:
					todo.put((neighbour_dist, neighbour[0]))
					open_set.add(neighbour[0])

	clique[start_vertex] = [(vertex, shortest_distance[vertex]) for vertex in range(len(adjacent)) if vertex < num_relevant and shortest_distance[vertex] != 0]
adjacent = clique

adjacent = list(adjacent[:num_relevant])  # Trim the vertices to just the relevant ones.
flow_rate = list(flow_rate[:num_relevant])
vertex_labels = list(vertex_labels[:num_relevant])
for i in range(len(adjacent)):
	adjacent[i] = [(vertex, length) for (vertex, length) in adjacent[i] if vertex < num_relevant]

@functools.lru_cache(maxsize=None)
def most_flow(time_left, current_pos, open_valves):
	"""
	Find the most flow rate we can add in a certain amount of time, while starting at the current position vertex, and
	already having opened a number of valves.
	:param time_left: The amount of time we have left, in minutes.
	:param current_pos: The current vertex we are at.
	:param open_valves: A bitfield of the valves that are opened so far.
	:return: The greatest amount of flow rate that we could yet add.
	"""
	if time_left <= 0:
		return 0  # No time to open additional valves.

	greatest_flow = 0
	for neighbour, distance in adjacent[current_pos]:
		if open_valves & (1 << neighbour) > 0:  # Already opened.
			continue
		if time_left <= distance + 1:  # Not enough time to go there and open the valve there.
			continue
		this_flow = most_flow(time_left - distance - 1, neighbour, open_valves | (1 << neighbour))
		this_flow += flow_rate[neighbour] * (time_left - distance - 1)
		if this_flow > greatest_flow:
			greatest_flow = this_flow
	return greatest_flow

start_open = 0
if flow_rate[-1] != 0:  # Never need to open this one since it has 0 flow. Pretend it's already open instead.
	start_open = 1 << num_relevant - 1
best = most_flow(30, num_relevant - 1, 0)
print("The greatest amount of flow you could open is:", best)