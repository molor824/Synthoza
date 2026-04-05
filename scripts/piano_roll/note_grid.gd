class_name NoteGrid extends Control

var _mouse_in := false
var _dragging := false
var _update_grid := true

@export var _opacity := 0.5
@export var _highlight_opacity := 1.0

@onready var _vertical_grid_scene := preload("res://scenes/piano_roll/vertical_grid.tscn")
@onready var _horizontal_grid_scene := preload("res://scenes/piano_roll/horizontal_grid.tscn")
@onready var _piano_roll := $"../../.." as PianoRoll
@onready var _vertical_container := $HBoxContainer as Control
@onready var _horizontal_container := $VBoxContainer as Control

func _process(_delta: float) -> void:
	if _update_grid:
		_update_grid = false

		var offset := _piano_roll.offset
		var note_size := _piano_roll.note_size
		var start := -Vector2(fposmod(offset.x, note_size.x), fposmod(offset.y, note_size.y))
		var index := Vector2i(offset / note_size)
		var count := Vector2i((size / note_size).ceil()) + Vector2i.ONE

		_vertical_container.position.x = start.x
		_horizontal_container.position.y = start.y

		for i in range(count.x):
			if i >= _vertical_container.get_child_count():
				_vertical_container.add_child(_vertical_grid_scene.instantiate())
			var grid := _vertical_container.get_child(i) as Control
			grid.custom_minimum_size.x = note_size.x
			grid.modulate.a = _opacity if (index.x + i) % _piano_roll.bars != 0 else _highlight_opacity
		for i in range(_horizontal_container.get_child_count(), count.y):
			if i >= _horizontal_container.get_child_count():
				_horizontal_container.add_child(_horizontal_grid_scene.instantiate())
			var grid := _horizontal_container.get_child(i) as Control
			grid.custom_minimum_size.y = note_size.y
			grid.modulate.a = _opacity
		
		for i in range(count.x, _vertical_container.get_child_count()):
			_vertical_container.get_child(i).queue_free()
		for i in range(count.y, _horizontal_container.get_child_count()):
			_horizontal_container.get_child(i).queue_free()

func _request_update_grid() -> void:
	_update_grid = true

func _ready() -> void:
	mouse_entered.connect(func() -> void: _mouse_in = true)
	mouse_exited.connect(func() -> void: _mouse_in = false)

	resized.connect(_request_update_grid)
	_piano_roll.offset_changed.connect(_request_update_grid)
	_piano_roll.note_size_changed.connect(_request_update_grid)
	_piano_roll.time_signature_changed.connect(_request_update_grid)

func _gui_input(event: InputEvent) -> void:
	if event.is_action_pressed("editor_drag") and _mouse_in:
		_dragging = true
	elif event.is_action_released("editor_drag"):
		_dragging = false
	var mouse_motion_event := event as InputEventMouseMotion
	if _dragging and mouse_motion_event != null:
		_piano_roll.offset.x -= mouse_motion_event.relative.x
