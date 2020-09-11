#!/usr/bin/env python3

import sys
import string

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
	"forest": "%",
	"swamp": "~",
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
<pre style="text-transform: full-width">
{}
</pre>
</body>
</html>
"""

def to_fullwidth(c):
	if c in string.ascii_letters + string.digits + string.punctuation:
		return chr(ord(c) - ord("!") + ord("！"))
	else:
		return c+c
	

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


def wrap_coordinates(chargrid, plot_size):
	plot_width, plot_height = plot_size
	topdigits = [" "] * len(chargrid[0])
	topcoords = [" "] * len(chargrid[0])
	for x in range(len(chargrid[0])):
		localpos = x%plot_height
		#plotpos = str(x - localpos)
		if localpos == 0:
			topcoords[x:x+len(str(x))] = [c for c in str(x)]
		topdigits[x] = str(localpos)
	#leftdigits = [str(y % plot_height) for y in range(len(chargrid))]
	for y in range(len(chargrid)):
		localpos = y%plot_height
		chargrid[y].insert(0, " ")
		chargrid[y].insert(0, str(localpos))
		chargrid[y].insert(0, " ")
		plotpos = str(y - localpos)
		chargrid[y].insert(0, plotpos[localpos] if localpos < len(plotpos) else " ")
	header = [" "]*4
	chargrid.insert(0, header + topdigits)
	chargrid.insert(0, header + topcoords)
	return chargrid


html = True
wide = True
coords = True

def main():
	field = Field(sys.stdin.read())
	grid = field.to_grid()
	chars = [[map_ent(ent, mapping) for ent in row] for row in grid]
	if coords:
		chars = wrap_coordinates(chars, field.plot_size)
	#if wide:
		#chars = [[to_fullwidth(char) for char in row] for row in chars]
	s = "\n".join("".join(line) for line in chars)
	if "html" in sys.argv:
		s = html_wrapper.format(s)
	print(s)

class Field:
	
	def __init__(self, inp):
		
		self.size = None
		self.plot_size = None
		self.tiles = {}
		
		meta, _, tiles = inp.partition("/")
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
