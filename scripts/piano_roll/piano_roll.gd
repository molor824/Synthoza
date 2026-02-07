class_name PianoRoll extends Control

@export var _piano: Piano

var _mouse_entered := false
var _dragging := false

func _ready() -> void:
	mouse_entered.connect(func(): _mouse_entered = true)
	mouse_exited.connect(func(): _mouse_entered = false)

func _drag(relative: Vector2) -> void:
	_piano.offset -= relative.y

func _gui_input(event: InputEvent) -> void:
	if event.is_action_pressed("editor_drag") and _mouse_entered:
		_dragging = true
	elif event.is_action_released("editor_drag"):
		_dragging = false
	var mouse_motion_event := event as InputEventMouseMotion
	if _dragging and mouse_motion_event != null:
		print(mouse_motion_event.relative)
		_drag(mouse_motion_event.relative)
