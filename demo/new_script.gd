extends Node

@export var script1: Rust

class test:
	var a: int
	func abc():
		pass

# Called when the node enters the scene tree for the first time.
func _ready():
	var singleton = Engine.get_singleton("RustLanguage")
	print(singleton)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
