class_name NewElementPopup extends Popup

@export var create_button: Button
@export var cancel_button: Button
@export var item_edit: LineEdit

signal element_created(value: String)

func on_create_with(value: String) -> void:
	element_created.emit(value)
	queue_free()

func on_create() -> void:
	on_create_with(item_edit.text)

func _ready() -> void:
	item_edit.text_submitted.connect(on_create_with)
	create_button.pressed.connect(on_create)
	cancel_button.pressed.connect(queue_free)
	close_requested.connect(queue_free)
	popup_hide.connect(queue_free)
