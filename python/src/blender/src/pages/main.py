import os
script_directory = os.path.dirname(os.path.abspath(__file__))
export_path = os.path.join(script_directory, "build")

# Blender Python の略
# Python から Blender を利用する場合はこのモジュールをインポートする必要がある
import bpy

# カメラの設定
camera = bpy.data.objects['Camera']
camera.location.x = 0.0
camera.location.y = 0.0
camera.location.z = 40.0
camera.rotation_euler.x = 0.0
camera.rotation_euler.y = 0.0
camera.rotation_euler.z = 0.0

# 既存のオブジェクトの削除
bpy.ops.object.delete()

# オブジェクトの作成

## Ruler === === ===
bpy.ops.curve.primitive_bezier_circle_add(radius=0.4, enter_editmode=False, location=(0, 0, 0))
bpy.ops.transform.resize(value=(6, 6, 6))
ruler_object = bpy.context.active_object
ruler_object.select_set(False)
## === === === === ===


## Page === === ===
bpy.ops.mesh.primitive_plane_add(size=4, enter_editmode=False, location=(0, 0, 0))
bpy.ops.transform.resize(value=(1.0, 1.0, 1.0))
# bpy.ops.transform.resize(value=(1.0, 1.414, 1.0))
bpy.context.active_object.name = "Page"
page_object = bpy.context.active_object
page_object.select_set(False)
## === === === === ===


## Instance === === ===
bpy.ops.mesh.primitive_plane_add(size=1, enter_editmode=False, location=(0, 0, 0))
parent_object = bpy.context.active_object

### Array の modifier を追加
bpy.ops.object.modifier_add(type='ARRAY')
bpy.context.object.modifiers["Array"].count = 80
bpy.context.object.modifiers["Array"].relative_offset_displace[0] = 0.2
bpy.context.object.modifiers["Array"].relative_offset_displace[1] = 0.2

### Circle の modifier を追加
bpy.ops.object.modifier_add(type='CURVE')
bpy.context.object.modifiers["Curve"].object = bpy.data.objects["BezierCircle"]
parent_object.select_set(False)
##  === === === === ===


# インスタンス化
page_object.parent = bpy.data.objects["Plane"]
parent_object.instance_type = 'FACES'
parent_object.show_instancer_for_viewport = False
parent_object.show_instancer_for_render = False


# 整形
page_object.select_set(True)
bpy.ops.object.mode_set(mode="EDIT")
bpy.ops.transform.resize(value=(1.0, 1.414, 1.0))
bpy.ops.transform.rotate(value=0.01, orient_axis='Y')
bpy.ops.transform.rotate(value=0.1, orient_axis='Z')
bpy.ops.object.mode_set(mode="OBJECT")
bpy.context.scene.render.use_freestyle = True
page_object.select_set(False)


# 画像の書き出し
bpy.context.scene.render.image_settings.color_mode = 'RGBA'
bpy.context.scene.render.film_transparent = True
image_path = os.path.join(export_path, "export_copy_object_sample.png")
bpy.ops.render.render()
bpy.data.images['Render Result'].save_render(image_path)


# ファイルをスクリプトのディレクトリに保存する
fpath = os.path.join(export_path, "export_sample.blend")
bpy.ops.wm.save_as_mainfile(filepath=fpath)
