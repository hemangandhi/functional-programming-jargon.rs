from sys import argv

class MarkdownHeadings:
    @staticmethod
    def into_heading_tree(md):
        root = MarkdownHeadings()
        in_snippet = False
        for line in md:
            if in_snippet and not line.startswith('```'):
                root.code_snippets[-1].append(line)
            elif line.startswith('```'):
                in_snippet = not in_snippet
                if in_snippet:
                    root.code_snippets.append([])
            elif not line.startswith('#'):
                root.lines.append(line)
            elif not in_snippet:
                level = line.count('#')
                heading = line.replace('#', '').strip()
                while root.level >= level: # by having a root of 0,
                                           # we can never run into
                                           # the None -- we know level >= 1
                    root = root.parent
                root[heading] = MarkdownHeadings(heading, root, level)
                root = root[heading]

        while root.parent is not None: root = root.parent

        return root

    def __init__(self, heading="", parent=None, level=None):
        self.lines = []
        self.heading = heading
        self.parent = parent
        self.sub_headings = dict()
        self.level = level if level is not None else\
                (0 if parent is None else parent.level + 1)
        self.code_snippets = []
    def __getitem__(self, item):
        if item in self.sub_headings:
            return self.sub_headings[item]
        else:
            return filter(lambda l: item in l, self.lines)
    def __setitem__(self, key, value):
        self.sub_headings[key] = value
    def __delitem__(self, key):
        del self.sub_headings[key]
    def __iter__(self):
        return iter(self.sub_headings)
    def to_link(self):
        return self.heading.lower().replace(' ', '-')
    def __str__(self):
        pre = "{} {}: [".format(''.join('#' for i in range(self.level)), self.heading)
        mid = ','.join(str(self[kid]) for kid in self)
        return pre + mid + ']'

def guess_rs_file_of_heading(heading, rs_file_base):
    def heading_to_path(heading_str, replace_space='-'):
        return heading_str.strip().lower().replace(' ', replace_space)

    path_guess = ''
    if heading.level > 2:
        tmp = heading.parent
        while tmp is not None:
            path_guess = heading_to_path(tmp.heading) + '/' + path_guess
    path_guess = rs_file_base + '/' + path_guess + heading_to_path(heading.heading, '_') + '_example.rs'
    return path_guess

def main(path, rs_file_base):
    with open(path) as readme:
        tree = MarkdownHeadings.into_heading_tree(readme)
        print(str(tree))
        tree = tree['Functional Programming Jargon in Rust']
        for lvl2 in tree.sub_headings:
            print(lvl2, guess_rs_file_of_heading(tree[lvl2], rs_file_base))

if __name__ == "__main__":
    main("../README.md", "../src")
