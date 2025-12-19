sample = """aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"""

file = open("input.txt")

def parse_device(line):
    devices = line.split(': ')
    connected = []
    for connection in devices[1].split():
        connected.append(connection)

    return devices[0], connected

def parse(input):
    devices = {}
    for line in input.splitlines():
        device, connected = parse_device(line)
        devices[device] = connected
    return devices

def path_seeker(current, devices):
    if current.lower() == "out":
        return 1

    total = 0
    connected = devices[current]
    for other in connected:
        result = path_seeker(other, devices)
        print(f"Result for {other} is {result}")
        total += result
    return total

rack = parse(file.read())
print(path_seeker("you", rack))