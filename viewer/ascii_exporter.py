#!/usr/bin/env python3

import sys

mapping = {
	"keep": 'K',
	"constuction": ":",
	"road": "/",
	"stockpile": "_",
	"stockpile:wood": "=",
	"stockpile:stone": "*",
	"stockpile:food": "%",
	"stockpile:iron": "-",
	"woodcutter": "W",
	"lair": "L",
	"farm": "F",
	"raider": "r",
	None: " "
}

html_wrapper = """
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>Evil Cadastre</title>
</head>
<body>
<pre>
{}
</pre>
</body>
</html>
"""


def map_ent(ent, mapping):
	if ent in mapping:
		return mapping[ent]
	elif ent.startswith("keep:"):
		return mapping["keep"]
	elif ent.startswith("construction:"):
		return mapping["constuction"]
	else:
		raise Exception("Unknown item: "+ent)

def parse_pos(s):
	xs, _, ys = s.partition(",")
	return (int(xs), int(ys))

def main():
	grid = Field(sys.stdin.read()).to_grid()
	chars = [[map_ent(ent, mapping) for ent in row] for row in grid]
	s = "\n".join("".join(line) for line in chars)
	if "html" in sys.argv:
		s = html_wrapper.format(s)
	print(s)

class Field:
	
	def __init__(self, inp):
		
		self.size = None
		self.plot_size = None
		self.tiles = {}
		
		meta, _, tiles = inp.partition("\n")
		for meta_item in meta.split():
			key, _, value = meta_item.partition(":")
			if key == "size":
				self.size = parse_pos(value)
			if key == "plot_size":
				self.plot_size = parse_pos(value)
		if self.size == None or self.plot_size == None:
			raise Exception("No size or plot size")
		for item in tiles.split(";"):
			item = item.strip()
			if item == "":
				return
			p, _, ent = item.partition(" ")
			pos = parse_pos(p)
			self.tiles[pos] = ent
	
	def to_grid(self):
		# indexed as grid[y][x]
		grid = [[None for x in range(self.size[0]*self.plot_size[0])] for y in range(self.size[1]*self.plot_size[1])]
		for ((x, y), ent) in self.tiles.items():
			grid[y][x] = ent
		return grid


if __name__ == "__main__":
	main()
