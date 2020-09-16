#!/usr/bin/env python3

import sys
import string
from evil import Field, map_ent, to_fullwidth


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
	args = set(sys.argv[1:])
	field = Field(sys.stdin.read())
	grid = field.to_grid()
	chars = [[map_ent(ent) for ent in row] for row in grid]
	if coords:
		chars = wrap_coordinates(chars, field.plot_size)
	if "wide" in args:
		chars = [[to_fullwidth(char) for char in row] for row in chars]
	s = "\n".join("".join(line) for line in chars)
	if "html" in args:
		s = html_wrapper.format(s)
	print(s)


if __name__ == "__main__":
	main()
