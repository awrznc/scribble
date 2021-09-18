import os
script_directory = os.path.dirname(os.path.abspath(__file__))

# Blender Python の略
# Python から Blender を利用する場合はこのモジュールをインポートする必要がある
import bpy


# 画像の書き出し
export_path = os.path.join(script_directory, "build")
image_path = os.path.join(export_path, "export_sample.png")

## ファイル形式をアルファチャネルにする
bpy.context.scene.render.image_settings.color_mode = 'RGBA'

## 背景透過
bpy.context.scene.render.film_transparent = True

## レンダリング
bpy.ops.render.render()

## 書き出し
bpy.data.images['Render Result'].save_render(image_path)



# カメラの設定
camera = bpy.data.objects['Camera']

## カメラの位置を変更 (location)
camera.location.x = 0.0
camera.location.y = 0.0
camera.location.z = 10.0

## カメラの角度を変更 (rotation_euler)
camera.rotation_euler.x = 0.0
camera.rotation_euler.y = 0.0
camera.rotation_euler.z = 1.0


# 画像の書き出し
image_path = os.path.join(export_path, "export_change_camera_sample.png")
bpy.ops.render.render()
bpy.data.images['Render Result'].save_render(image_path)



# 変形・移動

## G (移動。x方向に-0.5、y方向に0.5移動)
bpy.ops.transform.translate(value=(-0.5, 0.5, 0))

## R (回転)
bpy.ops.transform.rotate(value=0.5, orient_axis='Z')

## S (変形。立方体のサイズを1.5倍に変更)
bpy.ops.transform.resize(value=(1.5, 1.5, 1.5))


# 画像の書き出し
image_path = os.path.join(export_path, "export_change_shape_sample.png")
bpy.ops.render.render()
bpy.data.images['Render Result'].save_render(image_path)



# 既存のオブジェクトの削除
bpy.ops.object.delete()

# オブジェクトの作成
bpy.ops.mesh.primitive_plane_add(size=2, enter_editmode=False, location=(0, 0, 0))

# ワイヤーフレームの modifier を追加
bpy.ops.object.modifier_add(type='WIREFRAME')

# オブジェクトの複製
context = bpy.context
src_obj = bpy.context.active_object
src_obj.select_set(False)

for i in range (1,5):
    ## 複製
    new_obj = src_obj.copy()
    new_obj.data = src_obj.data.copy()
    new_obj.animation_data_clear()
    context.collection.objects.link(new_obj)

    ## 変形・移動
    new_obj.select_set(True)
    bpy.ops.transform.translate(value=(i*0.1, 0.0, 0.0))
    bpy.ops.transform.rotate(value=(i*0.1), orient_axis='Z')
    new_obj.select_set(False)

# オブジェクトの選択
# bpy.data.objects["Cube"].select_set(True)


# 画像の書き出し
image_path = os.path.join(export_path, "export_copy_object_sample.png")
bpy.ops.render.render()
bpy.data.images['Render Result'].save_render(image_path)



# ファイルをスクリプトのディレクトリに保存する
fpath = os.path.join(export_path, "export_sample.blend")
bpy.ops.wm.save_as_mainfile(filepath=fpath)
