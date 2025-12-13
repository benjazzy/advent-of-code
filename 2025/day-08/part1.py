import math

sample = """162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"""

file = open("input.txt")

def distance_between(a, b):
    x = (a[0] - b[0]) ** 2
    y = (a[1] - b[1]) ** 2
    z = (a[2] - b[2]) ** 2
    return math.sqrt(x + y + z)

def find_smallest_connections(count, boxes):
    connections = []
    for i, junction in enumerate(boxes):
        for other_junction in boxes[i + 1:]:
            distance = distance_between(junction, other_junction)
            connections.append({
                "dist_apart": distance,
                "a": junction,
                "b": other_junction
            })

    connections.sort(key=lambda connection: connection["dist_apart"])

    return connections[:count]

circuits = []
top_three = 0

boxes = []
# for line in sample.splitlines():
for line in file.readlines():
    coords = line.split(',')
    boxes.append((int(coords[0]), int(coords[1]), int(coords[2])))

for connection in find_smallest_connections(1000, boxes):
    print(f"Distance: {round(connection["dist_apart"], 2)}, A: {connection["a"]}, B: {connection["b"]}")
    a_circuit = None
    try:
        a_circuit = next(circuit for circuit in circuits if connection["a"] in circuit)
    except StopIteration:
        pass

    b_circuit = None
    try:
        b_circuit = next(circuit for circuit in circuits if connection["b"] in circuit)
    except StopIteration:
        pass



    if a_circuit is not None and b_circuit is None:
        a_circuit.add(connection["b"])
    elif a_circuit is None and b_circuit is not None:
        b_circuit.add(connection["a"])
    elif a_circuit is None and b_circuit is None:
        circuits.append({ connection["a"], connection["b"] })
    elif a_circuit == b_circuit:
        continue
    elif a_circuit is not None and b_circuit is not None:
        print(f"a_circuit: {a_circuit}, b_circuit: {b_circuit}")
        new_circuit = a_circuit.union(b_circuit)
        circuits.remove(a_circuit)
        circuits.remove(b_circuit)
        circuits.append(new_circuit)

circuits.sort(reverse=True, key=len)
print(circuits)

answer = 1
for size in map(len, circuits[:3]):
    answer *= size

print(answer)