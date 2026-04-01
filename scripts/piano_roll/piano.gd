class_name Piano extends Control

var _octave_scene := preload("res://scenes/piano_roll/octave.tscn")
var _offset_changed := true

@onready var _octave_scroll: Control = $OctaveScroll
@onready var _piano: PianoRoll = $"../.."

func _on_offset_change() -> void:
	_offset_changed = true

func _ready() -> void:
	resized.connect(_on_offset_change)
	_piano.offset_changed.connect(_on_offset_change)
	_piano.note_size_changed.connect(_on_offset_change)

func _process(_delta: float) -> void:
	if _offset_changed:
		var octave_height := _piano.note_size.y * 12
		var offset := _piano.offset.y
		_offset_changed = false
		_octave_scroll.position.y = -fposmod(offset, octave_height)
		var start_octave := floori(offset / octave_height)
		var end_octave := ceili((offset + size.y) / octave_height)
		var octaves := end_octave - start_octave
		var children := _octave_scroll.get_children()
		for i in octaves:
			var octave: Octave = children[i] if i < len(children) else _octave_scene.instantiate()
			octave.octave = i + start_octave
			if i >= len(children):
				_octave_scroll.add_child(octave)
		for i in range(octaves, len(children)):
			(children[i] as Control).queue_free()
