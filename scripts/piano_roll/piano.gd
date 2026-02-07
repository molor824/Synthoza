class_name Piano extends Control

var offset: float:
	get: return _offset
	set(value):
		if value != _offset:
			_offset = value
			_offset_changed = true
var octave_height: float:
	get: return _octave_height
	set(value):
		if value != _octave_height:
			_octave_height = value
			_offset_changed = true

var _octave_scene := preload("res://scenes/piano_roll/octave.tscn")
var _offset := 0.0
var _offset_changed := true
@export var _octave_height := 384.0

func _ready() -> void:
	resized.connect(func(): _offset_changed = true)

func _process(delta: float) -> void:
	if _offset_changed:
		_offset_changed = false
		var octave_scroll := $OctaveScroll
		octave_scroll.position.y = -fposmod(offset, octave_height)
		var start_octave := floori(offset / octave_height)
		var end_octave := ceili((offset + size.y) / octave_height)
		var octaves := end_octave - start_octave
		var children := octave_scroll.get_children()
		for i in octaves:
			var octave := children[i] if i < len(children) else _octave_scene.instantiate() as Octave
			octave.octave = i + start_octave
			octave.visible = true
			if i >= len(children):
				octave_scroll.add_child(octave)
		for i in range(octaves, len(children)):
			children[i].visible = false
