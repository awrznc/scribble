import os
script_directory = os.path.dirname(os.path.abspath(__file__))

import bpy
export_path = os.path.join(script_directory, "build")
image_path = os.path.join(export_path, "export_sample.png")

bpy.ops.render.render()
bpy.data.images['Render Result'].save_render(image_path)
