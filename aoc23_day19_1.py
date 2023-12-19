#!/usr/bin/env python


def process_target(name, part, rules, accepted, rejected):
    if name == 'R':
        rejected.append(part)
    elif name == 'A':
        accepted.append(part)
    else:
        for action in rules[name]:
            if action(part, rules, accepted, rejected):
                break


def parse_action(action_desc):
    if ':' not in action_desc:
        def do(part, rules, accepted, rejected):
            process_target(action_desc, part, rules, accepted, rejected)
            return True

        return do
    else:
        cond, target = action_desc.split(':')
        var_name = cond[0]
        val = int(cond[2:])
        if cond[1] == '<':
            cmp = lambda a, b: a < b
        else:
            cmp = lambda a, b: a > b

        def do(part, rules, accepted, rejected):
            if cmp(part.get(var_name), val):
                process_target(target, part, rules, accepted, rejected)
                return True

            return False

        return do


def read_rules():
    rules = {}
    while True:
        line = input().strip()
        if line == '':
            break

        rule_name, rest = line.split('{')
        rule_actions = []
        for action_desc in rest[:-1].split(','):
            rule_actions.append(parse_action(action_desc))

        rules[rule_name] = rule_actions

    return rules


def read_parts():
    parts = []
    while True:
        try:
            line = input().strip()
            part = {}
            for item in line[1:-1].split(','):
                key, val = item.split('=')
                part[key] = int(val)

            parts.append(part)
        except EOFError:
            break

    return parts


def main():
    accepted = []
    rejected = []
    rules = read_rules()
    parts = read_parts()
    for part in parts:
        process_target('in', part, rules, accepted, rejected)

    res = 0
    for part in accepted:
        for attr in ('x', 'm', 'a', 's'):
            res += part.get(attr)

    print(res)


if __name__ == '__main__':
    main()
