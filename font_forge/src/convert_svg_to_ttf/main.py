import fontforge

# create new font
font = fontforge.font()

# add charactor
charactor = '世'
glyph = font.createChar(ord(charactor), charactor)
glyph.importOutlines('hebi.svg')
glyph.left_side_bearing = 1
glyph.right_side_bearing = 1

# create truetype file
font.save("output.sfd")
font.generate('output.ttf')
