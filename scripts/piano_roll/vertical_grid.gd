class_name VerticalGrid extends Control

@export var _label: Label

var _measure := -1

var measure: int:
    get(): return _measure
    set(value):
        _measure = value
        _label.text = str(value) if value >= 0 else ""

func _ready() -> void:
    measure = _measure
