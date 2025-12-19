sample = """svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"""

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

def path_seeker(current, devices, cache, seen_dac = False, seen_fft = False):
    if current.lower() == "out":
        return 0, 1
    elif current.lower() == "dac":
        seen_dac = True
    elif current.lower() == "fft":
        seen_fft = True

    if current in cache:
        return cache[current]

    total = 0
    connected = devices[current]
    for other in connected:
        result = path_seeker(other, devices, cache, seen_dac, seen_fft)
        # print(f"Result for {other} is {result}")
        total += result

    if total > 0 and not (seen_dac or seen_fft):
        print(f"Caching {current} with value {total}")
        cache[current] = total

    return total

cache = {}
rack = parse(file.read())
print(path_seeker("svr", rack, cache))