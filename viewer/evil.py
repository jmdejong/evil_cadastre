#!/usr/bin/env python3

import string

mapping = {
	"capital": "@",
	"keep": "$",
	"constuction": ":",
	"road": "/",
	"stockpile": "_",
	"stockpile:wood": "=",
	"stockpile:stone": "*",
	"stockpile:food": "%",
	"stockpile:iron": "-",
	"woodcutter": "W",
	"farm": "F",
	"quarry": "Q",
	"lair": "L",
	"barracks": "B",
	"raider": "r",
	"warrior": "w",
	"ram": "a",
	"forest": "%",
	"swamp": "~",
	"rock": "^",
	None: " "
}


def to_fullwidth(c):
	if c in string.ascii_letters + string.digits + string.punctuation:
		return chr(ord(c) - ord("!") + ord("！"))
	else:
		return c+c
	

def map_ent(ent, charmap=mapping):
	if ent in mapping:
		return charmap[ent]
	elif ent.startswith("keep:"):
		return charmap["keep"]
	elif ent.startswith("capital:"):
		return charmap["capital"]
	elif ent.startswith("construction:"):
		return charmap["constuction"]
	else:
		raise Exception("Unknown item: "+ent)

def parse_pos(s):
	xs, _, ys = s.partition(",")
	return (int(xs), int(ys))


class Field:
	
	def __init__(self, inp):
		
		self.size = None
		self.plot_size = None
		self.tiles = {}
		
		meta, _, tiles = inp.partition(";;")
		for meta_item in meta.split(";"):
			key, _, value = meta_item.partition(":")
			key = key.strip()
			value = value.strip()
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
	
	def get(self, x, y):
		return self.tiles.get((x, y))
	
	def total_size(self):
		return (self.size[0] * self.plot_size[0], self.size[1] * self.plot_size[1])
	
	def to_grid(self):
		# indexed as grid[y][x]
		return [[self.get(x, y) for x in range(self.size[0]*self.plot_size[0])] for y in range(self.size[1]*self.plot_size[1])]
