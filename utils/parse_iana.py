import pathlib as pl
import csv
from collections import defaultdict


source = pl.Path('iana_ports.csv')
destination = pl.Path('iana.rs')

data = []
with source.open('r') as fd:
    reader = csv.reader(fd, delimiter=',')
    next(reader)
    for line in reader:
        data.append(line[:3])


pruned_data = [(protocol, ports, desc) for desc, ports, protocol in data]

mapping = defaultdict(dict)

for protocol, ports, desc in pruned_data:
    if not desc:
        continue
    single_ports = ports.strip().split('-')
    for port in single_ports:
        try:
            mapping[protocol][int(port)] = desc
        except Exception as e:
            print(port)

from pprint import pprint
pprint(mapping)

with destination.open('w') as fd:
    fd.write('use std::collections::HashMap;\n\n')
    for proto, map_ in mapping.items():
        if proto not in ('tcp', 'udp'):
            continue
        fd.write("""pub fn get_{}_map() -> HashMap<u64, &'static str> {}\n""".format(proto, '{'))
        fd.write('    let mut {}_iana_map = HashMap::new();\n\n'.format(proto))
        for port, desc in map_.items():
            fd.write('    {}_iana_map.insert({}, "{}");\n'.format(proto, port, desc))
        fd.write('    {}_iana_map\n'.format(proto))
        fd.write('}\n')
