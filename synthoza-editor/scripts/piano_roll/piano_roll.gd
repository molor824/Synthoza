class_name PianoRoll extends Control

@export var _note_size := Vector2(16 * 4, 16)
@export var _snap_value := 16 # Note at which the grid snaps
@export var _note_value := 4  # Base note for time signature
@export var _notes := 4       # Notes per signature

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

var notes: int:
	get(): return _notes
	set(value):
		value = maxi(value, 1)
		_notes = value
		time_signature_changed.emit()

var note_value: int:
	get(): return _note_value
	set(value):
		value = maxi(value, 1)
		_note_value = value
		time_signature_changed.emit()

var snap_value: int:
	get(): return _snap_value
	set(value):
		value = maxi(value, 1)
		_snap_value = value
		time_signature_changed.emit()
