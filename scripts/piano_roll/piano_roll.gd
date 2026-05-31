class_name PianoRoll extends Control

@export var _note_size := Vector2(24 * 4, 24)
@export var _bars := 4
@export var _crotchets := 4

var _mouse_entered := false
var _dragging := false
var _offset := Vector2.ZERO

signal note_size_changed()
signal offset_changed()
signal time_signature_changed()

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
        note_size_changed.emit()

var bars: int:
    get(): return _bars
    set(value):
        value = maxi(value, _bars)
        if value == _bars: return
        _bars = value
        time_signature_changed.emit()

var crotchets: int:
    get(): return _crotchets
    set(value):
        value = maxi(value, _crotchets)
        if value == _crotchets: return
        _crotchets = value
        time_signature_changed.emit()

func _on_mouse_enter() -> void:
    _mouse_entered = true

func _on_mouse_exit() -> void:
    _mouse_entered = false

func _ready() -> void:
    mouse_entered.connect(_on_mouse_enter)
    mouse_exited.connect(_on_mouse_exit)

func _gui_input(event: InputEvent) -> void:
    if event.is_action_pressed(InputActions.editor_drag):
        _dragging = true
    elif event.is_action_released(InputActions.editor_drag):
        _dragging = false
    
    var mouse_motion := event as InputEventMouseMotion
    if _dragging && mouse_motion:
        offset.y -= mouse_motion.relative.y
