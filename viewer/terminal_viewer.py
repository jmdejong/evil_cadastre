#!/usr/bin/env python3

import sys
import termios
import tty
import signal
import argparse
from ratuil.layout import Layout
from ratuil.screen import Screen
from ratuil.bufferedscreen import BufferedScreen
from ratuil.textstyle import TextStyle
from ratuil.inputs import get_key
from evil import Field, map_ent, to_fullwidth

layoutstring = """\
<?xml version="1.0"?>
<hbox>
	<vbox width="20" align="right">
		<textbox height="1"> Possible actions: </textbox>
		<textbox id="actions"></textbox>
	</vbox>
	
	<empty width="1" align="right">
	</empty>
	
	<vbox>
		<hbox height="1">
			<textbox width="5"> Pos: </textbox>
			<textbox id="position"></textbox>
		</hbox>
		<hbox height="1">
			<textbox width="6"> Tile: </textbox>
			<textbox id="viewed"></textbox>
		</hbox>
		<border style="reverse" char=" ">
			<field id="field" char-size="2"></field>
		</border>
	</vbox>
</hbox>
"""

class Entity:
	
	def __init__(self, char, style=None, actions=None):
		self.char = char
		self.style = style
		self.actions = actions if actions is not None else []

move = "m: Move"
#use = "p: Use"
attack = "f: Attack"
remove = "r: Remove"
build = "b: Build"

mapping = {
	"capital": Entity("＠", TextStyle(fg=15, bg=0), [move]),
	"keep": Entity("＄"),
	"constuction": Entity("::"),
	"road": Entity("//", TextStyle(3,7), [remove]),
	"tradepost": Entity("TT", TextStyle(3,7), [remove]),
	"stockpile": Entity("__", TextStyle(7), [remove]),
	"stockpile:wood": Entity("＝", TextStyle(underscore=True), [move, remove]),
	"stockpile:stone": Entity("＊", TextStyle(underscore=True), [move, remove]),
	"stockpile:food": Entity("88", TextStyle(underscore=True), [move, remove]),
	"stockpile:iron": Entity("－", TextStyle(underscore=True), [move, remove]),
	"scoutpost": Entity("Ｓ", actions=["u: Take over", remove]),
	"woodcutter": Entity("Ｗ", actions=["u: Produce Wood", remove]),
	"farm": Entity("Ｆ", actions=["u: Produce food", remove]),
	"quarry": Entity("Ｑ", actions=["u: Produce stone", remove]),
	"lair": Entity("Ｌ", actions=["u: Train raider", remove]),
	"barracks": Entity("Ｂ", actions=["u: Train warrior", remove]),
	"raider": Entity("ｒ", actions=[attack, move]),
	"warrior": Entity("ｗ", actions=[attack, move]),
	"ram": Entity("ａ", actions=[attack, remove]),
	"forest": Entity("&&", TextStyle(fg=3, bg=2)),
	"swamp": Entity("～", TextStyle(fg=10,bg=4)),
	"rock": Entity("^^", TextStyle(fg=7, bg=8)),
	None: Entity("  ", actions=[build])
}

# ！＂＃＄％＆＇（）＊＋，－．／０１２３４５６７８９：；＜＝＞？＠ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ［＼］＾＿｀ａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ｛｜｝～

base_style = TextStyle(fg=0, bg=10)

def clamp(n, lower, upper):
	return max(min(n, upper), lower)


class World:
	
	def __init__(self, field):
		self.field = field
		field_size = field.total_size()
		self.cursor = (field_size[0] // 2, field_size[1] // 2)
		self.commands = []
	
	def draw(self, layout):
		
		out = layout.get("field")
		field_size = self.field.total_size()
		out.set_size(*field_size)
		for x in range(field_size[0]):
			for y in range(field_size[1]):
				entdata = map_ent(self.field.get(x, y), mapping)
				out.change_cell(x, y, entdata.char+" ", base_style.add(entdata.style))
		out.set_center(*self.cursor)
		out.change_cell(*self.cursor, "}{", TextStyle(11, bold=True))
		layout.get("viewed").set_text(self.field.get(*self.cursor) or "empty")
		layout.get("position").set_text(",".join(str(c) for c in self.cursor))
		entdata = map_ent(self.field.get(*self.cursor), mapping)
		layout.get("actions").set_text("\n".join(entdata.actions))
		layout.update()
		
	def update(self, key):
		dx = (key in ("d", "right", "l")) - (key in ("a", "left", "h"))
		dy = (key in ("s", "down", "k")) - (key in ("w", "up", "l"))
		self.cursor = (clamp(self.cursor[0] + dx, 0, self.field.total_size()[0] - 1), clamp(self.cursor[1] + dy, 0, self.field.total_size()[1] - 1))


def parse_args():
	parser = argparse.ArgumentParser()
	parser.add_argument("--bbg", type=str, help="use blink attribute for bright backgrounds")
	parser.add_argument("world", type=argparse.FileType('r'), help="the file holding the evil cadastre world")
	#parser.add_argument("--hw", "--halfwidth", help="don't use fullwidth characters")
	return parser.parse_args()


def main():
	args = parse_args()
	scr = BufferedScreen(always_reset=True, blink_bright_background = args.bbg)
	scr.clear()
	
	layout = Layout.from_xml_str(layoutstring)
	
	layout.set_target(scr)
	layout.update(force=True)
	
	signal.signal(signal.SIGWINCH, (lambda signum, frame: scr.reset()))
	
	
	#layout.get("input").set_text("hello", 5)
	
	world = World(Field(args.world.read()))
	
	while True:
		world.draw(layout)
		if hasattr(scr, "update"):
			scr.update()
		inp = get_key(do_interrupt=True)
		#layout.get("messages").add_message(str(inp))
		world.update(inp)


if __name__ == "__main__":
	fd = sys.stdin.fileno()
	oldterm = termios.tcgetattr(fd)
	exitreason = None
	try:
		tty.setraw(sys.stdin)
		Screen.default.hide_cursor()
		main()
	except KeyboardInterrupt:
		exitreason = "^C caught, goodbye"
	finally:
		termios.tcsetattr(fd, termios.TCSADRAIN, oldterm)
		Screen.default.finalize()
	if exitreason:
		Screen.default.move(0, Screen.default.height-2)
		print(exitreason)
