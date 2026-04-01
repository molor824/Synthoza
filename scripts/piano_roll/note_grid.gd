class_name NoteGrid extends Control

@export var _bars := 4
@export var _note := 4

var _mouse_in := false
var _dragging := false
var _vertical_grids: Array[Control] = []
var _horizontal_grids: Array[Control] = []
var _update_grid := true

@onready var _vertical_grid_scene := preload("res://scenes/piano_roll/vertical_grid.tscn")
@onready var _horizontal_grid_scene := preload("res://scenes/piano_roll/horizontal_grid.tscn")
@onready var _piano_roll := $"../../.." as PianoRoll

func _process(_delta: float) -> void:
	if _update_grid:
		_update_grid = false

		var offset := _piano_roll.offset
		var note_size := _piano_roll.note_size
		var cursor := -Vector2(fposmod(offset.x, note_size.x), fposmod(offset.y, note_size.y))
		var vertical_count := 0
		var horizontal_count := 0
		
		while cursor.x <= size.x:
			while len(_vertical_grids) <= vertical_count:
				var copy := _vertical_grid_scene.instantiate()
				add_child(copy)
				_vertical_grids.append(copy)

			var grid := _vertical_grids[vertical_count]
			grid.position.x = cursor.x

			vertical_count += 1
			cursor.x += note_size.x
		while cursor.y <= size.y:
			while len(_horizontal_grids) <= horizontal_count:
				var copy := _horizontal_grid_scene.instantiate()
				add_child(copy)
				_horizontal_grids.append(copy)

			var grid := _horizontal_grids[horizontal_count]
			grid.position.y = cursor.y

			horizontal_count += 1
			cursor.y += note_size.y
		
		for i in range(vertical_count, len(_vertical_grids)):
			_vertical_grids[i].queue_free()
		for i in range(horizontal_count, len(_horizontal_grids)):
			_horizontal_grids[i].queue_free()
		
		_vertical_grids.resize(vertical_count)
		_horizontal_grids.resize(horizontal_count)

func _request_update_grid() -> void:
	_update_grid = true

func _ready() -> void:
	mouse_entered.connect(func() -> void: _mouse_in = true)
	mouse_exited.connect(func() -> void: _mouse_in = false)

	resized.connect(_request_update_grid)
	_piano_roll.offset_changed.connect(_request_update_grid)
	_piano_roll.note_size_changed.connect(_request_update_grid)

func _gui_input(event: InputEvent) -> void:
	if event.is_action_pressed("editor_drag") and _mouse_in:
		_dragging = true
	elif event.is_action_released("editor_drag"):
		_dragging = false
	var mouse_motion_event := event as InputEventMouseMotion
	if _dragging and mouse_motion_event != null:
		_piano_roll.offset.x -= mouse_motion_event.relative.x
