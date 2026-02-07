class_name Octave extends Control

@export var octave_label: Label
@export var _octave := 0

var octave: int:
	get:
		return _octave
	set(value):
		_octave = value
		octave_label.text = str(_octave)

func _ready() -> void:
	octave = _octave
