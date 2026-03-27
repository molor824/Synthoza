class_name NoteGrid extends Control

@export var _bars := 4
@export var _note := 4

var _mouse_in := false
var _dragging := false
var _x_offset := 0.0

var x_offset: float:
    get(): return _x_offset
    set(value):
        if _x_offset == value:
            return
        _x_offset = value
        x_offset_changed.emit()

signal x_offset_changed()

func _ready() -> void:
    mouse_entered.connect(func() -> void: _mouse_in = true)
    mouse_exited.connect(func() -> void: _mouse_in = false)

func _gui_input(event: InputEvent) -> void:
    if event.is_action_pressed("editor_drag") and _mouse_in:
        _dragging = true
    elif event.is_action_pressed("editor_drag"):
        _dragging = false
    var mouse_motion_event := event as InputEventMouseMotion
    if _dragging and mouse_motion_event != null:
        x_offset -= mouse_motion_event.relative.x
