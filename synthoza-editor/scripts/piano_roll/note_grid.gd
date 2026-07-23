class_name NoteGrid extends Control

const _FONT_SIZE := 14
const _PRIMARY_COLOR := Color(1, 1, 1, 0.3)
const _SECONDARY_COLOR := Color(1, 1, 1, 0.15)
const _SNAP_COLOR := Color(1, 1, 1, 0.05)

const _WHITE_KEY_BACKGROUND := Color(1, 1, 1, 0.05)
const _BLACK_KEY_BACKGROUND := Color(0, 0, 0, 0.05)

const _WHITE_KEY: Array[bool] = [true, false, true, false, true, true, false, true, false, true, false, true]

@export var _piano_roll: PianoRoll

func _ready() -> void:
	resized.connect(queue_redraw)
	_piano_roll.offset_changed.connect(queue_redraw)
	_piano_roll.note_size_changed.connect(queue_redraw)
	_piano_roll.time_signature_changed.connect(queue_redraw)

func _draw() -> void:
	var snap_value := maxi(_piano_roll.snap_value, _piano_roll.note_value)
	var snap_per_note := snap_value / _piano_roll.note_value
	var snap_per_sig := snap_per_note * _piano_roll.notes
	var note_size := _piano_roll.note_size
	var snap_size := note_size * Vector2(4.0 / float(snap_value), 1)
	var cursor := -(_piano_roll.offset).posmodv(snap_size)
	var grid_num := Vector2i(_piano_roll.offset / snap_size)
	var font := get_theme_default_font()
	
	while cursor.y < size.y:
		var y := size.y - cursor.y
		draw_rect(
			Rect2(0.0, y - note_size.y, size.x, note_size.y),
			_WHITE_KEY_BACKGROUND if _WHITE_KEY[posmod(grid_num.y, 12)] else _BLACK_KEY_BACKGROUND,
			true,
			-1,
			true
		)
		grid_num.y += 1
		cursor.y += snap_size.y
	
	while cursor.x < size.x:
		var snap_signature := posmod(grid_num.x, snap_per_sig)
		draw_line(
			Vector2(cursor.x, 0), Vector2(cursor.x, size.y),
			_PRIMARY_COLOR if snap_signature == 0 else (_SECONDARY_COLOR
				if posmod(snap_signature, snap_per_note) == 0 else
			_SNAP_COLOR),
			-1,
			true
		)
		if snap_signature == 0:
			var signature_num := int(grid_num.x / snap_per_sig) + 1
			draw_string(
				font,
				Vector2(cursor.x + 4, _FONT_SIZE),
				str(signature_num),
				HORIZONTAL_ALIGNMENT_LEFT,
				-1,
				_FONT_SIZE
			)
		
		grid_num.x += 1
		cursor.x += snap_size.x

func _gui_input(event: InputEvent) -> void:
	var mouse_motion := event as InputEventMouseMotion
	if Input.is_action_pressed(InputActions.editor_drag) && mouse_motion:
		_piano_roll.offset += mouse_motion.relative * Vector2(-1, 1)
