class_name List extends PanelContainer

@export var element_container: Node
@export var new_element_button: Button

var element_scene := preload("res://element.tscn")
var popup_scene := preload("res://new_element_popup.tscn")
var popup: Popup = null

func on_element_created(value: String) -> void:
	var element := element_scene.instantiate()
	element_container.add_child(element)
	element.name_label.text = value

func on_popup_cancelled() -> void:
	popup = null
	new_element_button.disabled = false

func on_new_element_pressed() -> void:
	if popup != null:
		return
	popup = popup_scene.instantiate()
	new_element_button.disabled = true
	add_child(popup)
	popup.element_created.connect(on_element_created)
	popup.tree_exiting.connect(on_popup_cancelled)

func _ready() -> void:
	new_element_button.pressed.connect(on_new_element_pressed)
