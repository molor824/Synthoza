class_name Element extends Panel

enum State {
	NORMAL,
	EDITING
}

@export var name_label: Label
@export var name_edit: LineEdit
@export var edit_button: Button
@export var delete_button: Button
@export var save_button: Button
@export var cancel_button: Button

var state := State.NORMAL

func sync() -> void:
	var is_normal := state == State.NORMAL
	var is_editing := state == State.EDITING
	
	name_label.visible = is_normal
	name_edit.visible = is_editing
	edit_button.visible = is_normal
	delete_button.visible = is_normal
	save_button.visible = is_editing
	cancel_button.visible = is_editing

func _on_delete() -> void:
	queue_free()

func _on_edit() -> void:
	state = State.EDITING
	name_edit.text = name_label.text
	sync()

func _on_save() -> void:
	_on_save_with(name_edit.text)

func _on_save_with(value: String) -> void:
	state = State.NORMAL
	name_label.text = value
	sync()

func _on_cancel() -> void:
	state = State.NORMAL
	sync()

func _ready() -> void:
	name_edit.text_submitted.connect(_on_save_with)
	edit_button.pressed.connect(_on_edit)
	delete_button.pressed.connect(_on_delete)
	save_button.pressed.connect(_on_save)
	cancel_button.pressed.connect(_on_cancel)
	sync()
