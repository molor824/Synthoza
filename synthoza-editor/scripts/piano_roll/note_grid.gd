class_name NoteGrid extends Control

var _mouse_in := false
var _dragging := false
var _change_requested := false

@export var _opacity := 0.5
@export var _highlight_opacity := 1.0
@export var _piano_roll: PianoRoll
@export var _vertical_container: HBoxContainer
@export var _horizontal_container: VBoxContainer
@export var _vertical_grid_scene: PackedScene
@export var _horizontal_grid_scene: PackedScene

func _request_change() -> void:
    _change_requested = true

func _ready() -> void:
    mouse_entered.connect(func() -> void: _mouse_in = true)
    mouse_exited.connect(func() -> void: _mouse_in = false)

    resized.connect(_request_change)
    _piano_roll.offset_changed.connect(_request_change)
    _piano_roll.note_size_changed.connect(_request_change)
    _piano_roll.time_signature_changed.connect(_request_change)

func _process(_delta: float) -> void:
    if !_change_requested: return
    _change_requested = false

    var offset := _piano_roll.offset
    var note_size := _piano_roll.note_size
    var start := -offset.posmodv(note_size)
    var index := Vector2i(offset / note_size)
    var count := Vector2i((size / note_size).ceil()) + Vector2i.ONE

    _vertical_container.position.x = start.x
    _horizontal_container.position.y = start.y

    var vertical_child_count := _vertical_container.get_child_count()
    for i in count.x:
        var grid: VerticalGrid
        if i < vertical_child_count:
            grid = _vertical_container.get_child(i)
        else:
            grid = _vertical_grid_scene.instantiate()
            _vertical_container.add_child(grid)

        grid.custom_minimum_size.x = note_size.x

        var bar := index.x + i
        grid.modulate.a = _opacity if bar % _piano_roll.bars != 0 else _highlight_opacity
        grid.measure = bar / _piano_roll.bars if bar % _piano_roll.bars == 0 else -1
    
    for i in range(count.x, vertical_child_count):
        _vertical_container.get_child(i).queue_free()
    
    var horizontal_child_count := _horizontal_container.get_child_count()
    for i in count.y:
        var grid: Control
        if i < horizontal_child_count:
            grid = _horizontal_container.get_child(i)
        else:
            grid = _horizontal_grid_scene.instantiate()
            _horizontal_container.add_child(grid)

        grid.custom_minimum_size.y = note_size.y
        grid.modulate.a = _opacity
    
    for i in range(count.y, horizontal_child_count):
        _horizontal_container.get_child(i).queue_free()

func _gui_input(event: InputEvent) -> void:
    if event.is_action_pressed(InputActions.editor_drag) && _mouse_in:
        _dragging = true
    elif event.is_action_released(InputActions.editor_drag):
        _dragging = false
    
    var mouse_motion := event as InputEventMouseMotion
    if _dragging && mouse_motion:
        _piano_roll.offset.x -= mouse_motion.relative.x
