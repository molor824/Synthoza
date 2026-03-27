class_name PianoRoll extends Control

var _mouse_entered := false
var _dragging := false
var _y_offset := 0.0

var y_offset: float:
	get(): return _y_offset
	set(value):
		if value == _y_offset: return
		_y_offset = value
		y_offset_changed.emit()

signal y_offset_changed()

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
		y_offset -= mouse_motion_event.relative.y
