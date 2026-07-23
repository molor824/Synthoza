class_name Piano extends Control

const _FONT_SIZE := 14

const _OUTLINE_WIDTH := 1.0
const _BLACK_KEY_RATIO := 0.6
const _BLACK_KEY_COLOR := Color(0.1, 0.1, 0.1)
const _WHITE_KEY_COLOR := Color(1, 1, 1)
const _OUTLINE_COLOR := Color.BLACK
const _WHITE_KEY_RECTS: Array[Rect2] = [
	Rect2(0, 0, 1, 1.5),
	Rect2(0, 1.5, 1, 2),
	Rect2(0, 3.5, 1, 2),
	Rect2(0, 5.5, 1, 1.5),
	Rect2(0, 7, 1, 1.5),
	Rect2(0, 8.5, 1, 2),
	Rect2(0, 10.5, 1, 1.5)
]
const _BLACK_KEY_RECTS: Array[Rect2] = [
	Rect2(0, 1, _BLACK_KEY_RATIO, 1),
	Rect2(0, 3, _BLACK_KEY_RATIO, 1),
	Rect2(0, 5, _BLACK_KEY_RATIO, 1),
	Rect2(0, 8, _BLACK_KEY_RATIO, 1),
	Rect2(0, 10, _BLACK_KEY_RATIO, 1)
]

@export var _piano_roll: PianoRoll

func _ready() -> void:
	resized.connect(queue_redraw)
	_piano_roll.offset_changed.connect(queue_redraw)
	_piano_roll.note_size_changed.connect(queue_redraw)

func _draw_key(rect: Rect2, color: Color) -> void:
	draw_rect(rect, _OUTLINE_COLOR, true, -1, true)
	draw_rect(
		Rect2(
			rect.position + Vector2.ONE * _OUTLINE_WIDTH,
			rect.size - Vector2.ONE * (2 * _OUTLINE_WIDTH)
		), color, true, -1, true
	)

func _draw() -> void:
	var key_size := Vector2(size.x, _piano_roll.note_size.y)
	var octave_size := key_size.y * 12
	var cursor := -fposmod(_piano_roll.offset.y, octave_size)
	var octave := int(_piano_roll.offset.y / octave_size)
	var font := get_theme_default_font()
	
	while cursor <= size.y:
		var offset := Vector2.DOWN * (size.y - cursor - octave_size)
		for rect in _WHITE_KEY_RECTS:
			_draw_key(Rect2(
				rect.position * key_size + offset,
				rect.size * key_size
			), _WHITE_KEY_COLOR)
		for rect in _BLACK_KEY_RECTS:
			_draw_key(Rect2(
				rect.position * key_size + offset,
				rect.size * key_size
			), _BLACK_KEY_COLOR)
		
		var width := font.get_string_size(str(octave))
		draw_string(
			font,
			Vector2(key_size.x - width.x, key_size.y * 12 - 4) + offset,
			str(octave),
			HORIZONTAL_ALIGNMENT_LEFT,
			-1,
			_FONT_SIZE,
			Color.BLACK
		)
		
		cursor += octave_size
		octave += 1

func _gui_input(event: InputEvent) -> void:
	var mouse_motion := event as InputEventMouseMotion
	if Input.is_action_pressed(InputActions.editor_drag) && mouse_motion:
		_piano_roll.offset.y += mouse_motion.relative.y
