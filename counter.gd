class_name Counter extends HBoxContainer

@export var _decrement: Button
@export var _increment: Button
@export var _label: Label

var _count := 0

func sync() -> void:
	_label.text = "Count: %s" % _count

func _on_decrement() -> void:
	_count -= 1
	sync()
func _on_increment() -> void:
	_count += 1
	sync()

func _ready() -> void:
	_decrement.pressed.connect(_on_decrement)
	_increment.pressed.connect(_on_increment)
	sync()
