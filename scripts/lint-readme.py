from sys import argv

class MarkdownHeadings:
    @staticmethod
    def into_heading_tree(md):
        root = MarkdownHeadings()
        for line in md:
            if not line.startswith('#'):
                root.lines.append(line)
            else:
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
    def __getitem__(self, item):
        if item in self.sub_headings:
            return self.sub_headings[item]
        else:
            return filter(lambda l: item in l, self.lines)
    def __setitem__(self, key, value):
        self.sub_headings[key] = value
    def __delitem__(self, key):
        del self.sub_headings[key]
    def to_link(self):
        return self.heading.lower().replace(' ', '-')

def main(path):
    with open(path) as readme:

