class_name PianoRoll extends Control

@export var _note_size := Vector2(100, 32)

var _mouse_entered := false
var _dragging := false
var _offset := Vector2.ZERO

var offset: Vector2:
	get(): return _offset
	set(value):
		value = value.max(Vector2.ZERO)
		if value == _offset: return
		_offset = value
		offset_changed.emit()

var note_size: Vector2:
	get(): return _note_size
	set(value):
		value = value.max(Vector2.ZERO)
		if value == _note_size: return
		_note_size = value
		note_size_changed.emit()

signal note_size_changed()
signal offset_changed()

func _ready() -> void:
	mouse_entered.connect(func() -> void: _mouse_entered = true)
	mouse_exited.connect(func() -> void: _mouse_entered = false)

func _gui_input(event: InputEvent) -> void:
	if event.is_action_pressed("editor_drag") and _mouse_entered:
		_dragging = true
	elif event.is_action_released("editor_drag"):
		_dragging = false
	var mouse_motion_event := event as InputEventMouseMotion
	if _dragging and mouse_motion_event != null:
		offset.y -= mouse_motion_event.relative.y
