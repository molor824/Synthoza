class_name Octave extends Control

@export var _label: Label
@export var _index := 0

var index: int:
    get(): return _index
    set(value):
        _index = value
        _label.text = str(value)

func _ready() -> void:
    _label.text = str(index)
