class_name Piano extends Control

@export var _octave_scroll: VBoxContainer
@export var _piano_roll: PianoRoll
@export var _octave_scene: PackedScene

var _change_requested := true

func _request_change() -> void:
	_change_requested = true

func _ready() -> void:
	resized.connect(_request_change)
	_piano_roll.offset_changed.connect(_request_change)
	_piano_roll.note_size_changed.connect(_request_change)

func _process(_delta: float) -> void:
	if !_change_requested: return
	_change_requested = false

	var octave_height := _piano_roll.note_size.y * 12
	var offset := _piano_roll.offset.y

	_octave_scroll.position.y = -fposmod(offset, octave_height)
	
	var start_octave := floori(offset / octave_height)
	var end_octave := ceili((offset + size.y) / octave_height)
	var octaves := end_octave - start_octave
	var child_count := _octave_scroll.get_child_count()

	for i in octaves:
		var octave: Octave
		if i < child_count:
			octave = _octave_scroll.get_child(i)
		else:
			octave = _octave_scene.instantiate()
			_octave_scroll.add_child(octave)
		
		octave.index = i + start_octave
		octave.custom_minimum_size.y = octave_height
	
	for i in range(octaves, child_count):
		_octave_scroll.get_child(i).queue_free()
